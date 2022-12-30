use crud_api::{Api, ApiInput, ApiRun, Query};
use crud_auth::CrudAuth;
use crud_auth_no_auth::Auth;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// http://jsonplaceholder.typicode.com/
#[derive(Api, Debug, Deserialize, Serialize, Default)]
#[api(endpoint(route = "/", cli_route = "/foo", payload_struct = "Enum"))]
#[allow(dead_code, non_snake_case)]
struct Foo {}

// #[derive(Api, Debug, Deserialize, Serialize)]
// #[api(endpoint(route = "/", cli_route = "/enum",))]
// #[allow(dead_code, non_snake_case)]
// enum Enum {
//   A,
//   B(Bstruct),
//   C(u32),
//   D { d: u32, dd: String },
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Bstruct {
//   ba: u32,
//   bb: String,
// }

#[derive(ApiRun)]
#[api(infos(
  base_url = "http://localhost:3210",
  name = "enum",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "ENUM"
))]
struct R {}

#[derive(Debug, Default, ApiInput, Deserialize, Serialize)]
struct Bstruct {
  #[api(no_short)]
  ba: u32,
  #[api(no_short)]
  bb: String,
}

#[derive(ApiInput, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[api(no_input_file)]
#[allow(dead_code)]
enum Enum {
  #[api(no_short)]
  A,
  #[api(no_short)]
  B(Bstruct),
  // #[api(no_short)]
  // C(u32),
  // #[api(no_short)]
  // D { d: u32, dd: String },
}

impl Default for Enum {
  fn default() -> Self {
    Enum::A
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let e = Enum::A;
  println!("{:?}", e);

  R::run().await
  //  Ok(())
}
