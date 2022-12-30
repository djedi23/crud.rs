use crud_api::{Api, ApiInput, ApiRun, Query};
use crud_auth::CrudAuth;
use crud_auth_bearer::Auth;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Api, Debug, Default, Deserialize, Serialize)]
#[api(
  endpoint(route = "/issues", multiple_results, cli_route = "/issues",),
  endpoint(route = "/issues/{id}", cli_route = "/issues/{id}")
)]
#[api(
  endpoint(
    route = "/projects/{id}/issues",
    multiple_results,
    cli_route = "/projects/{id}/issues",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}",
    cli_route = "/projects/{id}/issues/{iid}"
  ),
  endpoint(
    route = "/projects/{id}/issues",
    cli_route = "/projects/{id}/issues/create",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "IssueCreatePayload"
  )
)]
#[allow(dead_code, non_snake_case)]
struct Issue {
  id: u32,
  state: String,
  project_id: u32,
  iid: u32,
  title: String,
  #[api(table_skip)]
  description: Option<String>,
  #[api(format(date(format = "%Y-%m-%d %H:%M:%S")))]
  updated_at: String,
  #[api(table_skip)]
  web_url: String,
  #[api(format(date(format = "%Y-%m-%d %H:%M:%S")))]
  created_at: String,
  labels: Vec<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
struct IssueCreatePayload {
  #[api(no_short)]
  title: String,
  description: Option<String>,
  labels: Option<Vec<String>>,
}

#[derive(ApiRun)]
#[api(infos(
  base_url = "https://gitlab.com/api/v4",
  name = "gitlab",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "GITLAB"
))]
struct R {}

#[tokio::main]
async fn main() -> Result<()> {
  R::run().await
}
