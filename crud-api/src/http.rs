use crate::{error::ApiError, Query};
use async_trait::async_trait;
use crud_auth::CrudAuth;
use http::uri::Authority;
use http_body_util::BodyExt;
use hyper::client::conn::http1::SendRequest;
use hyper::{body::Buf, Method, Request, StatusCode, Uri};
use hyper_util::rt::TokioIo;
use indicatif::{ProgressBar, ProgressStyle};
use log::trace;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use serde::{de::DeserializeOwned, Serialize};
use std::{
  collections::HashMap, fmt::Debug, io::Read, marker::PhantomData, path::Path, sync::Arc,
  time::Duration,
};
use tokio::{
  fs::{create_dir_all, File},
  io::{stdout, AsyncWriteExt},
  net::TcpStream,
};
use tokio_rustls::{
  rustls::{ClientConfig, RootCertStore},
  TlsConnector,
};

pub struct HTTPApi<'a> {
  uri: String,
  method: Method,
  ok_status: StatusCode,
  ko_status: &'a HashMap<StatusCode, String>,
  auth: Option<&'a (dyn CrudAuth + Send + Sync)>,
  headers: &'a Vec<Header<'a>>,
}

impl<'a> HTTPApi<'a> {
  pub fn new(
    uri: String,
    method: Method,
    ok_status: StatusCode,
    ko_status: &'a HashMap<StatusCode, String>,
    auth: Option<&'a (dyn CrudAuth + Send + Sync)>,
    headers: &'a Vec<Header<'a>>,
  ) -> HTTPApi<'a> {
    HTTPApi {
      uri,
      method,
      ok_status,
      ko_status,
      auth,
      headers,
    }
  }
}

#[derive(Clone)]
pub struct Header<'a> {
  pub key: &'a str,
  pub value: &'a str,
}

async fn connect(uri: &str) -> Result<(SendRequest<String>, Authority)> {
  let url: Uri = uri
    .parse()
    .into_diagnostic()
    .context("Error during URL parsing")?;

  let host = url.host().expect("uri has no host");
  let port = url.port_u16().unwrap_or({
    if let Some(scheme) = url.scheme() {
      trace!("scheme {:?}", scheme);
      if scheme == &http::uri::Scheme::HTTPS {
        443
      } else {
        80
      }
    } else {
      80
    }
  });
  let addr = format!("{}:{}", host, port);

  let mut root_cert_store = RootCertStore::empty();
  root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
  let config = ClientConfig::builder()
    .with_root_certificates(root_cert_store)
    .with_no_client_auth();
  let connector = TlsConnector::from(Arc::new(config));

  let domain = url.host().unwrap();
  let domain = pki_types::ServerName::try_from(domain)
    .into_diagnostic()?
    .to_owned();

  let stream = TcpStream::connect(addr).await.into_diagnostic()?;
  let sender = if let Some(scheme) = url.scheme() {
    if scheme == &http::uri::Scheme::HTTPS {
      let io = TokioIo::new(connector.connect(domain, stream).await.into_diagnostic()?);
      let (sender, conn) = hyper::client::conn::http1::handshake(io)
        .await
        .into_diagnostic()?;
      tokio::task::spawn(async move {
        if let Err(err) = conn.await {
          println!("Connection failed: {:?}", err);
        }
      });
      sender
    } else {
      let io = TokioIo::new(stream);
      let (sender, conn) = hyper::client::conn::http1::handshake(io)
        .await
        .into_diagnostic()?;
      tokio::task::spawn(async move {
        if let Err(err) = conn.await {
          println!("Connection failed: {:?}", err);
        }
      });
      sender
    }
  } else {
    let io = TokioIo::new(stream);
    let (sender, conn) = hyper::client::conn::http1::handshake(io)
      .await
      .into_diagnostic()?;
    tokio::task::spawn(async move {
      if let Err(err) = conn.await {
        println!("Connection failed: {:?}", err);
      }
    });
    sender
  };
  // The authority of our URL will be the hostname of the httpbin remote
  let authority = url.authority().unwrap().clone();

  Ok((sender, authority))
}

#[async_trait]
impl Query for HTTPApi<'_> {
  async fn query<P, T, R, Q>(
    &self,
    payload: Option<P>,
    query_args: Option<Q>,
    transform_from_type: Option<PhantomData<T>>,
  ) -> Result<R>
  where
    P: Send + Serialize + Debug,
    T: TryInto<R, Error = String> + DeserializeOwned + Send,
    R: Send + DeserializeOwned + Debug + Default,
    Q: Send + Serialize + Debug,
  {
    let mut uri = self.uri.to_owned();
    if let Some(qa) = query_args {
      uri = format!("{}?{}", uri, serde_qs::to_string(&qa).unwrap());
    }
    let (mut sender, authority) = connect(&uri).await?;

    let req = Request::builder().method(&self.method).uri(&uri);
    trace!("Request {} to {}", self.method, uri);
    let req = if let Some(auth) = self.auth {
      let (header_key, header_value) = auth.auth_header();
      if !header_key.is_empty() {
        req.header(header_key, header_value)
      } else {
        req
      }
    } else {
      req
    };
    let mut req = req;
    for Header { key, value } in self.headers.iter() {
      req = req.header(*key, *value)
    }

    let req = req
      .header("content-type", "application/json; charset=UTF-8")
      .header(hyper::header::HOST, authority.as_str())
      .body(match payload {
        Some(ref payload) => {
          let p = serde_json::to_string(&payload)
            .into_diagnostic()
            .context("Error during payload serialization")?;
          trace!("Payload: {}", p);
          p
        }
        None => String::new(),
      })
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP request preparation failed.")?;

    let response = sender
      .send_request(req)
      .await
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP call fail")?;
    trace!("Response status: {}", response.status());
    if response.status() == self.ok_status {
      let body = response
        .collect()
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .context("Can't read the HTTP response")?
        .aggregate();

      if !body.has_remaining() {
        Ok(R::default()) // I don't find a type that can deserialize an empty string.
      } else {
        #[cfg(feature = "debug-http")]
        {
          let mut buffer = String::new();
          body
            .reader()
            .read_to_string(&mut buffer)
            .into_diagnostic()
            .wrap_err("Can't read error as string")?;
          println!("{}", buffer);
          let result: R = if transform_from_type.is_some() {
            let raw_result: T = serde_json::from_str(&buffer)
              .into_diagnostic()
              .context("Can't deserialize the response")?;
            raw_result.try_into().map_err(|e| miette!("{}", e))?
          } else {
            serde_json::from_str(&buffer)
              .into_diagnostic()
              .context("Can't deserialize the response")?
          };
          Ok(result)
        }
        #[cfg(not(feature = "debug-http"))]
        {
          let result: R = if transform_from_type.is_some() {
            let raw_result: T = serde_json::from_reader(body.reader())
              .into_diagnostic()
              .context("Can't deserialize the response")?;
            raw_result.try_into().map_err(|e| miette!("{}", e))?
          } else {
            serde_json::from_reader(body.reader())
              .into_diagnostic()
              .context("Can't deserialize the response")?
          };
          Ok(result)
        }
      }
    } else {
      let empty_string = String::default();
      let message = self
        .ko_status
        .get(&response.status())
        .unwrap_or(&empty_string)
        .to_string();

      trace!("Response {:?}", response);

      let status = response.status();

      let mut error_body = String::new();
      response
        .collect()
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .wrap_err("Can't read the HTTP error response")?
        .aggregate()
        .reader()
        .read_to_string(&mut error_body)
        .into_diagnostic()
        .wrap_err("Can't read error as string")?;
      #[cfg(feature = "debug-http")]
      {
        println!("{}", error_body);
      }
      Err(ApiError::from_http_status(status, self.auth))
        .wrap_err(error_body)
        .wrap_err_with(|| format!("URL: {uri}"))
        .wrap_err(if message.is_empty() {
          "Unexpected HTTP Status Code".to_string()
        } else {
          message
        })?
    }
  }

  async fn stream<P, Q>(
    &self,
    payload: Option<P>,
    query_args: Option<Q>,
    filename: Option<String>,
  ) -> Result<()>
  where
    P: Send + Serialize + Debug,
    Q: Send + Serialize + Debug,
  {
    let mut uri = self.uri.to_owned();
    let (mut sender, authority) = connect(&uri).await?;

    if let Some(qa) = query_args {
      uri = format!("{}?{}", uri, serde_qs::to_string(&qa).unwrap());
    }
    let req = Request::builder().method(&self.method).uri(&uri);
    trace!("Request {} to {}", self.method, uri);
    let req = if let Some(auth) = self.auth {
      let (header_key, header_value) = auth.auth_header();
      if !header_key.is_empty() {
        req.header(header_key, header_value)
      } else {
        req
      }
    } else {
      req
    };
    let mut req = req;
    for Header { key, value } in self.headers.iter() {
      req = req.header(*key, *value)
    }
    let req = req
      .header("content-type", "application/json")
      .header(hyper::header::HOST, authority.as_str())
      .body(match payload {
        Some(ref payload) => {
          let p = serde_json::to_string(&payload)
            .into_diagnostic()
            .context("Error during payload serialization")?;
          trace!("Payload: {}", p);
          p
        }
        None => String::new(),
      })
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP request preparation failed.")?;

    let mut response = sender
      .send_request(req)
      .await
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP call fail")?;
    trace!("Response status: {}", response.status());
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(150));
    bar.set_style(
      ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg} {bytes:>12} @ {bytes_per_sec:15} ({elapsed})")
        .into_diagnostic()?
        .tick_strings(&[
          "▹▹▹▹▹",
          "▸▹▹▹▹",
          "▹▸▹▹▹",
          "▹▹▸▹▹",
          "▹▹▹▸▹",
          "▹▹▹▹▸",
          "▪▪▪▪▪",
        ]),
    );
    bar.set_message("Downloading...");
    if response.status() == self.ok_status {
      // use hyper::body::HttpBody;
      // use tokio::io::{stdout, AsyncWriteExt as _};
      // while let Some(chunk) = response.body_mut().data().await {
      //   stdout().write_all(&chunk?).await?;
      // }
      if let Some(path) = filename {
        let path = Path::new(&path);
        if let Some(dir) = path.parent() {
          create_dir_all(dir)
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
        }
        let mut file = File::create(path)
          .await
          .into_diagnostic()
          .with_context(|| format!("Failed to create file: {}", path.display()))?;

        while let Some(chunk) = response.frame().await {
          let frame = chunk
            .into_diagnostic()
            .with_context(|| format!("URL: {uri}"))?;
          if let Some(chunk) = frame.data_ref() {
            bar.inc(chunk.len().try_into().into_diagnostic()?);
            file
              .write_all(chunk)
              .await
              .into_diagnostic()
              .with_context(|| format!("Failed to write file: {}", path.display()))?;
          }
        }
      } else {
        while let Some(chunk) = response.frame().await {
          let frame = chunk
            .into_diagnostic()
            .with_context(|| format!("URL: {uri}"))?;
          if let Some(chunk) = frame.data_ref() {
            bar.inc(chunk.len().try_into().into_diagnostic()?);
            stdout()
              .write_all(chunk)
              .await
              .into_diagnostic()
              .context("Failed to write: {}")?;
          }
        }
      };
      bar.finish_and_clear();
      Ok(())
    } else {
      let empty_string = String::default();
      let message = self
        .ko_status
        .get(&response.status())
        .unwrap_or(&empty_string)
        .to_string();

      let status = response.status();
      let mut error_body = String::new();
      response
        .collect()
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .wrap_err("Can't read the HTTP error response")?
        .aggregate()
        .reader()
        .read_to_string(&mut error_body)
        .into_diagnostic()
        .wrap_err("Can't read error as string")?;

      Err(ApiError::from_http_status(status, self.auth))
        .wrap_err(error_body)
        .wrap_err_with(|| format!("URL: {uri}"))
        .wrap_err(if message.is_empty() {
          "Unexpected HTTP Status Code".to_string()
        } else {
          message
        })?
    }
  }
}
