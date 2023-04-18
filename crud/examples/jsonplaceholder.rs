use crud::Crud;
use crud_api::{Api, ApiInput, ApiRun, EmptyResponse, Query};
use crud_auth::CrudAuth;
use crud_auth_no_auth::Auth;
use miette::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, ApiInput, Deserialize, Serialize, Default)]
#[api(heading = "Filters")]
struct PostParameters {
  #[api(help = "Filter by post author's id")]
  #[serde(rename = "userId")]
  user_id: Option<u32>,
  #[api(help = "Filter by title of the post", no_short)]
  title: Option<String>,
  #[api(help = "Filter by body of the post")]
  body: Option<String>,
}

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(parameters = "PostParameters")]
#[crud(nested(route = "/users/{id}/posts"))]
#[allow(dead_code, non_snake_case)]
struct Posts {
  #[crud(id, no_short)]
  id: u32,
  #[crud(help = "post author's id")]
  #[serde(rename = "userId")]
  user_id: u32,
  #[crud(help = "title of the post")]
  #[crud(no_short)]
  title: String,
  #[crud(help = "body of the post")]
  body: String,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
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

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
struct Geo {
  #[api(no_short)]
  lat: String,
  #[api(no_short)]
  lng: String,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
struct Address {
  street: String,
  #[api(no_short)]
  suite: String,
  city: String,
  zipcode: String,
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

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(route = "/users")]
struct User {
  #[crud(id, no_short)]
  id: u32,
  name: String,
  username: String,
  email: String,
  phone: String,
  website: String,
  company: Company,
  address: Address,
}

#[derive(Debug, ApiInput, Deserialize, Serialize, Default)]
#[api(heading = "Filters")]
struct CommentFilters {
  #[serde(rename = "postId")]
  post_id: Option<u32>,
  name: Option<String>,
  email: Option<String>,
  body: Option<String>,
}

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(route = "/comments", parameters = "CommentFilters")]
#[crud(nested(route = "/posts/{id}/comments"))]
#[allow(dead_code, non_snake_case)]
struct Comment {
  #[serde(rename = "postId")]
  post_id: u32,
  #[crud(id, no_short)]
  id: u32,
  name: String,
  email: String,
  body: String,
}

#[derive(Debug, ApiInput, Deserialize, Serialize, Default)]
#[api(heading = "Filters")]
struct AlbumFilters {
  #[serde(rename = "userId")]
  user_id: Option<u32>,
  title: Option<String>,
}

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(route = "/albums", parameters = "AlbumFilters", help = "User's album")]
#[crud(nested(route = "/users/{id}/albums"))]
struct Album {
  #[crud(id, no_short)]
  id: u32,
  #[serde(rename = "userId")]
  user_id: u32,
  #[crud(no_short)]
  title: String,
}

#[derive(Debug, ApiInput, Deserialize, Serialize, Default)]
#[api(heading = "Filters")]
struct PhotoFilters {
  #[serde(rename = "albumId")]
  album_id: Option<u32>,
  #[api(no_short)]
  title: Option<String>,
  url: Option<String>,
  #[serde(rename = "thumbnailUrl")]
  #[api(no_short)]
  thumbnail_url: Option<String>,
}

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(route = "/photos", parameters = "PhotoFilters")]
#[crud(nested(route = "/albums/{id}/photos"))]
struct Photo {
  #[crud(id, no_short)]
  id: u32,
  #[serde(rename = "albumId")]
  album_id: u32,
  title: String,
  url: String,
  #[serde(rename = "thumbnailUrl")]
  #[crud(no_short)]
  thumbnail_url: String,
}

#[derive(Debug, ApiInput, Deserialize, Serialize, Default)]
#[api(heading = "Filters")]
struct TodoFilters {
  #[serde(rename = "userId")]
  user_id: Option<u32>,
  #[api(no_short)]
  title: Option<String>,
  completed: Option<bool>,
}

#[derive(Debug, Crud, Deserialize, Serialize, Default)]
#[crud(route = "/todos", parameters = "TodoFilters")]
#[crud(nested(route = "/users/{id}/todos"))]
struct Todo {
  #[crud(id, no_short)]
  id: u32,
  #[serde(rename = "userId")]
  user_id: u32,
  #[crud(no_short)]
  title: String,
  completed: bool,
}

// http://jsonplaceholder.typicode.com/
#[derive(ApiRun)]
#[api(infos(
  base_url = "http://jsonplaceholder.typicode.com",
  name = "jsonplaceholder",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "JSONPLACEHOLDER"
))]
struct JsonPlaceHolder {}

#[tokio::main]
async fn main() -> Result<()> {
  //println!("{:?}", Post::list().await?);
  JsonPlaceHolder::run().await?;
  Ok(())
}
