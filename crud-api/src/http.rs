use crate::{error::ApiError, Query};
use async_trait::async_trait;
use crud_auth::CrudAuth;
use hyper::{
  body::{aggregate, Buf, HttpBody},
  client::HttpConnector,
  Body, Client, Method, Request, StatusCode,
};
use hyper_tls::HttpsConnector;
use indicatif::{ProgressBar, ProgressStyle};
use log::trace;
use miette::{IntoDiagnostic, Result, WrapErr};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fmt::Debug, io::Read, path::Path, time::Duration};
use tokio::{
  fs::{create_dir_all, File},
  io::{stdout, AsyncWriteExt},
};

pub struct HTTPApi<'a> {
  uri: String,
  method: Method,
  ok_status: StatusCode,
  ko_status: &'a HashMap<StatusCode, String>,
  auth: &'a (dyn CrudAuth + Send + Sync),
}

impl<'a> HTTPApi<'a> {
  pub fn new(
    uri: String,
    method: Method,
    ok_status: StatusCode,
    ko_status: &'a HashMap<StatusCode, String>,
    auth: &'a (dyn CrudAuth + Send + Sync),
  ) -> HTTPApi<'a> {
    HTTPApi {
      uri,
      method,
      ok_status,
      ko_status,
      auth,
    }
  }
}

fn get_client() -> Client<HttpsConnector<HttpConnector>> {
  let https = HttpsConnector::new();
  Client::builder().build::<_, hyper::Body>(https)
}

#[async_trait]
impl Query for HTTPApi<'_> {
  async fn query<P, R, Q>(&self, payload: Option<P>, query_args: Option<Q>) -> Result<R>
  where
    P: Send + Serialize + Debug,
    R: Send + DeserializeOwned + Debug + Default,
    Q: Send + Serialize + Debug,
  {
    let client = get_client();
    let mut uri = self.uri.to_owned();
    if let Some(qa) = query_args {
      uri = format!("{}?{}", uri, serde_qs::to_string(&qa).unwrap());
    }
    let req = Request::builder().method(&self.method).uri(&uri);
    trace!("Request {} to {}", self.method, uri);
    let (header_key, header_value) = self.auth.auth_header();
    let req = if !header_key.is_empty() {
      req.header(header_key, header_value)
    } else {
      req
    };
    let req = req
      .header("content-type", "application/json; charset=UTF-8")
      .body(match payload {
        Some(ref payload) => Body::from({
          let p = serde_json::to_string(&payload)
            .into_diagnostic()
            .context("Error during payload serialization")?;
          trace!("Payload: {}", p);
          p
        }),
        None => Body::empty(),
      })
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP request preparation failed.")?;

    let response = client
      .request(req)
      .await
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP call fail")?;
    trace!("Response status: {}", response.status());
    // {
    // use hyper::body::HttpBody;
    // use tokio::io::{stdout, AsyncWriteExt as _};
    // while let Some(chunk) = response.body_mut().data().await {
    //   stdout()
    //     .write_all(&chunk.into_diagnostic()?)
    //     .await
    //     .into_diagnostic()?;
    // }
    if response.status() == self.ok_status {
      let body = aggregate(response)
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .context("Can't read the HTTP response")?;

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
          //          println!("{}", buffer);
          let result: R = serde_json::from_str(&buffer)
            .into_diagnostic()
            .context("Can't deserialize the response")?;
          Ok(result)
        }
        #[cfg(not(feature = "debug-http"))]
        {
          let result: R = serde_json::from_reader(body.reader())
            .into_diagnostic()
            .context("Can't deserialize the response")?;
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

      let status = response.status();

      let mut error_body = String::new();
      aggregate(response)
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .wrap_err("Can't read the HTTP error response")?
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
    let client = get_client();
    let mut uri = self.uri.to_owned();
    if let Some(qa) = query_args {
      uri = format!("{}?{}", uri, serde_qs::to_string(&qa).unwrap());
    }
    let req = Request::builder().method(&self.method).uri(&uri);
    trace!("Request {} to {}", self.method, uri);
    let (header_key, header_value) = self.auth.auth_header();
    let req = if !header_key.is_empty() {
      req.header(header_key, header_value)
    } else {
      req
    };
    let req = req
      .header("content-type", "application/json")
      .body(match payload {
        Some(ref payload) => Body::from({
          let p = serde_json::to_string(&payload)
            .into_diagnostic()
            .context("Error during payload serialization")?;
          trace!("Payload: {}", p);
          p
        }),
        None => Body::empty(),
      })
      .into_diagnostic()
      .with_context(|| format!("Payload: {payload:?}"))
      .with_context(|| format!("URL: {uri}"))
      .context("HTTP request preparation failed.")?;

    let mut response = client
      .request(req)
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

        while let Some(chunk) = response.data().await {
          let chunk = chunk
            .into_diagnostic()
            .with_context(|| format!("URL: {uri}"))?;
          bar.inc(chunk.len().try_into().into_diagnostic()?);
          file
            .write_all(&chunk)
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        }
      } else {
        while let Some(chunk) = response.data().await {
          let chunk = chunk
            .into_diagnostic()
            .with_context(|| format!("URL: {uri}"))?;
          bar.inc(chunk.len().try_into().into_diagnostic()?);
          stdout()
            .write_all(&chunk)
            .await
            .into_diagnostic()
            .context("Failed to write: {}")?;
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
      aggregate(response)
        .await
        .into_diagnostic()
        .with_context(|| format!("URL: {uri}"))
        .wrap_err("Can't read the HTTP error response")?
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
