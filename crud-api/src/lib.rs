//! # Crud Api
//! 
//! This crate provides a framework to generate an executable to manipulate your HTTP API from CLI.
//! 
//! The apps using this lib can replace your _curl_ queries when you need to access to your favorite API.
//! 
//! ## Features
//! 
//! API:
//! - data are encoded in JSON. It don't support XML, grpc, ...
//! - output can be formated on json, yaml, toml, csv or tsv
//! - output stream on stdout or in a file
//! 
//! 
//! ## Tutorial
//! 
//! Let's create an CLI for [jsonplaceholder](http://jsonplaceholder.typicode.com/) API.
//! For the impatients, the whole code of this example can be found in [`examples/jsonplaceholder_api.rs`](./examples/jsonplaceholder_api.rs "jsonplaceholder_api.rs")
//! 
//! First add these dependencies to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! log = "0.4"
//! pretty_env_logger = "0.5"
//! clap = "4.3"
//! crud-api = {version = "0.1", path="../crud/crud-api", default-features=false, features=["toml","json","yaml"]}
//! crud-auth = {version = "0.1", path="../crud/crud-auth"}
//! crud-auth-bearer = {version = "0.1", path="../crud/crud-auth-bearer"}
//! hyper = { version = "0.14", features = ["client","http1"] }
//! hyper-tls = "0.5"
//! miette = { version = "5.9", features = ["fancy"] }
//! tokio = { version = "1", features = ["full"] }
//! serde = { version = "1.0", features = ["derive"] }
//! # To force static openssl
//! openssl = { version = "0.10", features = ["vendored"] }
//! ```
//! 
//! Now, create a minimal runner stucture and a `main` function.
//! `ApiRun` on `JSONPlaceHolder` derives all the CLI.
//! ```rust
//! use crud_api::ApiRun;
//! use crud_auth::CrudAuth;
//! use crud_auth_no_auth::Auth;
//! use miette::{IntoDiagnostic, Result};
//! 
//! #[derive(ApiRun)]
//! struct JSONPlaceHolder;
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   JSONPlaceHolder::run().await
//! }
//! ```
//! [`crud_api_endpoint::ApiRun`] accepts some parameters. They are documented in `crud_api_endoint` crate.
//! Let's customize our CLI with a `base_url` for our API, a `name` used in the documentation and the settings. `qualifier` and `organisation` is used to compute the settings location and `env_prefix` is the prefix of the environment variables
//! ```rust
//! # use crud_api::ApiRun;
//! # use crud_auth::CrudAuth;
//! # use crud_auth_no_auth::Auth;
//! # use miette::{IntoDiagnostic, Result};
//! #[derive(ApiRun)]
//! #[api(infos(
//!   base_url = "http://jsonplaceholder.typicode.com",
//!   name = "jsonplaceholder",
//!   qualifier = "com",
//!   organisation = "typicode",
//!   env_prefix = "JSONPLACEHOLDER"
//! ))]
//! struct JSONPlaceHolder;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! #   JSONPlaceHolder::run().await
//! # }
//! ```
//! Before creating the first endpoint we need to describe its output structure.
//! ```rust
//! use serde::{Deserialize, Serialize};
//! #[derive(Debug, Default, Deserialize, Serialize)]
//! struct Post {
//!   id: u32,
//!   #[serde(rename = "userId")]
//!   user_id: u32,
//!   title: String,
//!   body: String,
//! }
//! ```
//! 
//! Now, we can declare the endpoint.
//! The minimal parameters are:
//! - `route`, the target api route.
//! - `cli_route`, the route transcipted as cli arguments. Each slash separate a subcommand.
//! The other parameters can found in [`crud_api_endpoint::Api`] and [`crud_api_endpoint::Enpoint`] structs documentation.
//! 
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! use crud_api::Api;
//! #[derive(Api, Debug, Default, Deserialize, Serialize)]
//! #[api(
//!   endpoint(
//!     route = "/posts",
//!     cli_route = "/post",
//!     multiple_results,
//!   ))]
//! struct Post {
//!   id: u32,
//!   #[serde(rename = "userId")]
//!   user_id: u32,
//!   title: String,
//!   body: String,
//! }
//! ```
//! We can create more complex enpoint. Let's create an edit route.
//! 
//! - The `route` parameter takes a post's `id` argument. This argument should be present in the `cli_route`.
//! - the HTTP method is set with the `method` parameter.
//! - Some help can be provided via the parameters `cli_help` and `cli_long_help`.
//! - the payload is described by the struct declared with the `payload_struct`. The query parameter can be add with the `query_struct` parameter.
//! 
//! In this step, the payload structure is `PostCreate` (the same structure is used for both creation and update). `PostCreate` derives `ApiInput`. All `PostCreate` fields parameters are describe in the [`crud_api_endpoint::ApiInputConfig`] structs.
//! 
//! 
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! use crud_api::{Api, ApiInput};
//! #[derive(Api, Debug, Default, Deserialize, Serialize)]
//! #[api(
//!   endpoint(
//!     route = "/posts",
//!     cli_route = "/post",
//!     multiple_results,
//!   ),
//!   endpoint(
//!     route = "/posts/{id}",
//!     method = "PUT",
//!     payload_struct = "PostCreate",
//!     cli_route = "/post/{id}/replace",
//!     cli_help = "Update a Posts (replace the whole post)"
//!   )
//! )]
//! struct Post {
//!   id: u32,
//!   #[serde(rename = "userId")]
//!   user_id: u32,
//!   title: String,
//!   body: String,
//! }
//! 
//! #[derive(Debug, ApiInput, Default, Serialize, Deserialize)]
//! #[allow(dead_code, non_snake_case)]
//! struct PostCreate {
//!   #[api(long = "user-id")]
//!   userId: u32,
//!   #[api(no_short, help = "Title of the post")]
//!   title: String,
//!   #[api(no_short)]
//!   body: String,
//! }
//! ```
//! 
//! ## Output Customization
//! 
//! ### Tables
//! 
//! Results arrays are formatted using the crate [`crud-tidy-viewer`](crud_tidy_viewer).
//! The available table column options are:
//! - [`table_skip`](../crud_api_endpoint/struct.ApiField.html#structfield.table_skip): don't display this field in the table.
//! - [`table_format`](../crud_api_endpoint/struct.ApiField.html#structfield.table_format): format this field in table.
//!   - date formatter: `date(format = "%Y-%m-%d %H:%M:%S")`
//! 
//! ### Pretty Structures
//! 
//! The crate [`crud-pretty-struct`](crud_pretty_struct) can format a single (json) struct.

use async_trait::async_trait;
use clap::{ArgMatches, Command, Id};
pub use crud_api_derive::*;
use crud_pretty_struct::PrettyPrint;
#[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
use crud_tidy_viewer::{display_table, TableConfig};
use formats::OutputFormat;
#[doc(hidden)]
pub use formats::{
  clap_match_input_from_file, clap_match_output_format, clap_match_template, clap_output_format_decl,
};
use miette::{IntoDiagnostic, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
  fmt::Debug,
  marker::{PhantomData, Sized},
};

extern crate crud_api_derive;
#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub mod completions;
#[doc(hidden)]
pub mod error;
mod formats;
#[doc(hidden)]
pub mod http;
#[doc(hidden)]
pub mod settings;

#[doc(hidden)]
pub struct ApiInputOptions {
  pub conflicts_with_all: Vec<Id>,
}

#[doc(hidden)]
pub trait ApiInput {
  /// Generate the clap command declatations.
  fn clap(app: Command, options: Option<ApiInputOptions>) -> Command;
  fn from_clap_matches(matches: &ArgMatches) -> Result<Self>
  where
    Self: Sized;
}

#[doc(hidden)]
#[async_trait]
pub trait Query {
  async fn query<P, T, R, Q>(
    &self,
    payload: Option<P>,
    argument: Option<Q>,
    t: Option<PhantomData<T>>,
  ) -> Result<R>
  where
    P: Send + Serialize + Debug,
    T: TryInto<R, Error = String> + DeserializeOwned + Send,
    R: Send + DeserializeOwned + Debug + Default,
    Q: Send + Serialize + Debug;
  async fn stream<P, Q>(
    &self,
    payload: Option<P>,
    argument: Option<Q>,
    filename: Option<String>,
  ) -> Result<()>
  where
    P: Send + Serialize + Debug,
    Q: Send + Serialize + Debug;
}

#[doc(hidden)]
pub trait Api {
  fn to_table_header(&self) -> Vec<String>;
  fn to_table(&self) -> Result<Vec<String>>;
  fn to_output(&self) -> Result<String>;

  #[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
  fn output(&self, format: Option<OutputFormat>) -> Result<()>
  where
    Self: Serialize + Debug,
  {
    let out = match format {
      Some(format) => match format {
        #[cfg(feature = "json")]
        OutputFormat::Json => Some(serde_json::to_string_pretty(self).into_diagnostic()?),
        #[cfg(feature = "toml")]
        OutputFormat::Toml => Some(toml::to_string(self).into_diagnostic()?),
        #[cfg(feature = "yaml")]
        OutputFormat::Yaml => Some(serde_yaml::to_string(self).into_diagnostic()?),
        #[cfg(feature = "csv")]
        OutputFormat::Csv => {
          let mut wtr = csv::Writer::from_writer(std::io::stdout());
          wtr.serialize(self).into_diagnostic()?;
          wtr.flush().into_diagnostic()?;
          None
        }
        #[cfg(feature = "csv")]
        OutputFormat::Tsv => {
          let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(std::io::stdout());
          wtr.serialize(self).into_diagnostic()?;
          wtr.flush().into_diagnostic()?;
          None
        }
      },
      None => Some(self.to_output()?),
    };

    if let Some(out) = out {
      print!("{out}");
    }
    Ok(())
  }

  #[cfg(all(
    not(feature = "json"),
    not(feature = "toml"),
    not(feature = "yaml"),
    not(feature = "csv")
  ))]
  fn output(&self, _format: Option<OutputFormat>) -> Result<()>
  where
    Self: Serialize + Debug,
  {
    Ok(())
  }

  #[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
  fn output_multiple(results: &Vec<Self>, format: Option<OutputFormat>) -> Result<()>
  where
    Self: Sized + Serialize + Debug,
  {
    let out = match format {
      Some(format) => match format {
        #[cfg(feature = "json")]
        OutputFormat::Json => Some(serde_json::to_string_pretty(results).into_diagnostic()?),
        #[cfg(feature = "toml")]
        OutputFormat::Toml => Some(toml::to_string(results).into_diagnostic()?),
        #[cfg(feature = "yaml")]
        OutputFormat::Yaml => Some(serde_yaml::to_string(results).into_diagnostic()?),
        #[cfg(feature = "csv")]
        OutputFormat::Csv => {
          let mut wtr = csv::Writer::from_writer(std::io::stdout());
          for result in results {
            wtr.serialize(result).into_diagnostic()?;
          }
          wtr.flush().into_diagnostic()?;
          None
        }
        #[cfg(feature = "csv")]
        OutputFormat::Tsv => {
          let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(std::io::stdout());
          for result in results {
            wtr.serialize(result).into_diagnostic()?;
          }
          wtr.flush().into_diagnostic()?;
          None
        }
      },
      None => {
        if !results.is_empty() {
          let mut table = vec![results.iter().next().unwrap().to_table_header()];
          table.append(
            &mut results
              .iter()
              .map(|row| row.to_table().expect("Formating data table"))
              .collect(),
          ); // FIXME: replace expect by something better from miette
          display_table(&table, TableConfig::default());
        }
        None
      }
    };
    if let Some(out) = out {
      println!("{out}");
    }
    Ok(())
  }

  #[cfg(all(
    not(feature = "json"),
    not(feature = "toml"),
    not(feature = "yaml"),
    not(feature = "csv")
  ))]
  fn output_multiple(_results: &Vec<Self>, _format: Option<OutputFormat>) -> Result<()>
  where
    Self: Sized + Serialize + Debug,
  {
    Ok(())
  }
}

/// An empty response. Use it in `result_struct`
#[derive(Debug, Default, Deserialize, Serialize, PrettyPrint)]
pub struct EmptyResponse {}
impl Api for EmptyResponse {
  fn to_table_header(&self) -> Vec<String> {
    vec![]
  }

  fn to_table(&self) -> Result<Vec<String>> {
    Ok(vec![])
  }

  fn to_output(&self) -> Result<String> {
    Ok(String::new())
  }
}

#[derive(Deserialize)]
pub struct DummyTryFrom;

impl TryFrom<DummyTryFrom> for EmptyResponse {
  type Error = String;
  fn try_from(_value: DummyTryFrom) -> std::result::Result<Self, Self::Error> {
    Err(String::new())
  }
}

impl<T> TryFrom<DummyTryFrom> for Vec<T> {
  type Error = String;
  fn try_from(_value: DummyTryFrom) -> std::result::Result<Self, Self::Error> {
    Err(String::new())
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
