use crud_api::{Api, ApiInput, ApiRun, EmptyResponse, Query};
use crud_auth::CrudAuth;
use crud_auth_no_auth::Auth;
use crud_pretty_struct::PrettyPrint;
use miette::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

// http://jsonplaceholder.typicode.com/
#[derive(Api, Debug, Default, Deserialize, Serialize)]
#[api(
  endpoint(
    route = "/posts",
    multiple_results,
    cli_route = "/post",
    query_struct = "PostFilters"
  ),
  endpoint(route = "/posts/{id}", cli_route = "/post/{id}"),
  endpoint(
    route = "/users/{user_id}/posts",
    multiple_results,
    cli_route = "/user/{user_id}/post"
  )
)]
#[api(endpoint(
  route = "/posts",
  method = "POST",
  payload_struct = "PostCreate",
  result_ok_status = "CREATED",
  cli_route = "/post/create",
  cli_visible_aliases = "add,insert",
  cli_long_flag_aliases = "create,add,insert",
  cli_aliases = "c,a,i",
  cli_short_flag_aliases = "c,a,i"
))]
#[api(endpoint(
  route = "/posts/{id}",
  method = "DELETE",
  result_struct = "EmptyResponse",
  cli_route = "/post/{id}/delete"
))]
#[api(
  endpoint(
    route = "/posts/{id}",
    method = "PUT",
    payload_struct = "PostCreate",
    cli_route = "/post/{id}/replace",
    cli_help = "Update a Posts (replace the whole post)"
  ),
  endpoint(
    route = "/posts/{id}",
    method = "PATCH",
    payload_struct = "PostUpdate",
    cli_route = "/post/{id}/update",
    cli_help = "Update a Posts"
  )
)]
#[derive(PrettyPrint)]
#[allow(dead_code, non_snake_case)]
struct Post {
  id: u32,
  userId: u32,
  title: String,
  body: String,
}

#[derive(Debug, ApiInput, Default, Serialize, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct PostCreate {
  #[api(long = "user-id")]
  userId: u32,
  #[api(help = "Title of the post")]
  #[api(no_short)]
  title: String,
  #[api(no_short)]
  body: String,
}
#[derive(Debug, ApiInput, Default, Serialize, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct PostUpdate {
  #[api(long = "user-id")]
  #[serde(skip_serializing_if = "Option::is_none")]
  userId: Option<u32>,
  #[api(help = "Title of the post")]
  #[api(no_short)]
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,
  #[api(no_short)]
  #[serde(skip_serializing_if = "Option::is_none")]
  body: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
#[allow(dead_code, non_snake_case)]
struct PostFilters {
  #[api(long = "user-id", heading = "Filters", help = "Filter by user-id")]
  userId: Option<u32>,
  // #[api(help = "filter by title", heading = "Filters")]
  // title: String,
  // #[api(no_short, heading = "Filters")]
  // body: String,
}

#[derive(Api, Debug, Default, Deserialize, Serialize)]
#[api(endpoint(
  route = "/comments",
  multiple_results,
  cli_route = "/comment",
  query_struct = "CommentFilters"
))]
#[allow(dead_code, non_snake_case)]
struct Comment {
  id: u32,
  postId: u32,
  name: String,
  email: String,
  body: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
#[allow(non_snake_case)]
struct CommentFilters {
  #[api(long = "post-id", heading = "Filters", help = "Filter by post-id")]
  postId: Option<u32>,
  #[api(heading = "Filters", help = "Filter by emil")]
  email: Option<String>,
}

#[derive(Api, Debug, Default, Serialize, Deserialize)]
#[api(endpoint(route = "/users", multiple_results, cli_route = "/user",))]
#[api(endpoint(route = "/users/{user_id}", cli_route = "/user/{user_id}",))]
#[api(endpoint(
  route = "/users",
  method = "POST",
  cli_route = "/user/create",
  payload_struct = "UserCreate",
  result_ok_status = "CREATED",
))]
#[derive(PrettyPrint)]
//#[allow(non_snake_case)]
struct User {
  id: u32,
  name: String,
  username: String,
  email: String,
  phone: String,
  website: String,
  #[pretty(is_pretty)]
  company: Company,
  #[pretty(is_pretty)]
  address: Address,
}
#[derive(Debug, Default, ApiInput, Serialize, Deserialize, PrettyPrint)]
struct Geo {
  #[api(no_short)]
  lat: String,
  #[api(no_short)]
  lng: String,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize, PrettyPrint)]
struct Address {
  street: String,
  #[api(no_short)]
  suite: String,
  city: String,
  zipcode: String,
  #[pretty(is_pretty)]
  geo: Geo,
}
impl Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{}; {}; {}",
      self.street, self.zipcode, self.city
    ))
  }
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize, PrettyPrint)]
#[allow(non_snake_case)]
struct Company {
  #[api(no_short)]
  name: String,
  #[api(no_short)]
  catchPhrase: String,
  #[api(no_short)]
  bs: String,
}
impl Display for Company {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}", self.name))
  }
}

#[derive(Debug, ApiInput, Default, Serialize, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct UserCreate {
  name: String,
  username: String,
  email: String,
  address: Address,
  phone: String,
  website: String,
  company: Company,
}

#[derive(ApiRun)]
#[api(infos(
  base_url = "http://jsonplaceholder.typicode.com",
  name = "jsonplaceholder",
  about = "jsonplaceholder cli example.",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "JSONPLACEHOLDER"
))]
struct R {}

#[tokio::main]
async fn main() -> Result<()> {
  R::run().await
}
