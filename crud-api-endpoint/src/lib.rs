//! Tools crate for `crud-api` and `crud` crates.
//!
//!

mod api;
mod api_run;
mod config;
mod input;
mod types;

pub use api::{table_impl, Api, ApiField, ApiVariant, FieldFormat};
pub use api_run::{ApiInformation, ApiRun};
pub use config::{arg_config, ApiInputConfig};
use darling::FromMeta;
use derive_builder::Builder;
pub use input::{ApiInputFieldSerde, ApiInputSerde, ApiInputVariantSerde, DataSerde};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};
pub use types::VecStringWrapper;

#[macro_use]
extern crate lazy_static;

/// Specify an Http endpoint
#[derive(Debug, Clone, Builder, FromMeta, Serialize, Deserialize)]
#[builder(setter(into))]
#[builder(default)]
#[darling(default)]
pub struct Endpoint {
  /// Absolute route as format template
  /// Variables are written in curly braces `{}`.
  ///
  /// Examples:
  /// ```text
  /// /root/{id}/sub/{arg}
  /// ```
  #[serde(skip_serializing_if = "String::is_empty")]
  pub route: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub method: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub payload_struct: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub query_struct: Option<String>,
  // #[serde(skip_serializing_if = "String::is_empty")]
  // pub attributes_struct: String,
  /// Expected status if query is ok
  #[serde(skip_serializing_if = "String::is_empty")]
  pub result_ok_status: String,
  #[darling(multiple)]
  pub result_ko_status: Vec<EndpointStatus>,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub result_struct: String,
  /// returns a list of results
  #[darling(rename = "multiple_results")]
  pub result_multiple: bool,
  /// returns a stream of bytes for this endpoint
  /// This flag generates the `--output` arguments.
  /// This flag disables the `--format` arguments.
  #[darling(rename = "stream")]
  pub result_is_stream: bool,

  /// clap route separated by slash (`/`)
  ///
  /// Variables should match the variables declared in the `route` configuration.
  /// ```text
  /// /command/{id}/subcommand/{arg}
  /// ```
  #[serde(skip_serializing_if = "String::is_empty")]
  pub cli_route: String,
  /// Short help string for this endpoint
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_help: Option<String>,
  /// Long help string for this endpoint.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_long_help: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_visible_aliases: Option<VecStringWrapper>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_long_flag_aliases: Option<VecStringWrapper>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_aliases: Option<VecStringWrapper>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_short_flag_aliases: Option<VecStringWrapper>,
  /// This empty have no output to display.
  /// It can be combined with the `EmptyResponse` result structure.
  ///
  /// Examples:
  /// ```text
  /// endpoint(
  ///   result_ok_status = "NO_CONTENT",
  ///   cli_no_output,
  ///   result_struct = "EmptyResponse",
  ///   route = "...",
  ///   cli_route = "...",
  /// ),
  /// ```
  pub cli_no_output: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cli_output_formats: Option<VecStringWrapper>,
  /// Force the generation of '--format' args in variable sub command.
  /// There's cases where the arg is not generated automatically.
  ///
  /// Example:
  /// ```text
  /// /route/{var}'
  /// ```
  /// By default, `{var}` don't generate `--format`.
  /// If route is just a passthrough, you need the `cli_force_output_format` to generate
  /// the `--format` args.
  pub cli_force_output_format: bool,

  #[darling(default)]
  #[darling(multiple)]
  pub config: Vec<ApiInputConfig>,
}

#[derive(Debug, Clone, Default, Builder, FromMeta, Serialize, Deserialize)]
#[builder(setter(into))]
#[builder(default)]
#[darling(default)]
pub struct EndpointStatus {
  pub status: String,
  pub message: String,
}

impl Default for Endpoint {
  fn default() -> Self {
    Self {
      method: "GET".into(),
      route: Default::default(),
      payload_struct: Default::default(),
      query_struct: Default::default(),
      //      attributes_struct: Default::default(),
      result_ok_status: "OK".into(),
      result_ko_status: Default::default(),
      result_struct: Default::default(),
      result_multiple: Default::default(),
      result_is_stream: false,
      cli_route: Default::default(),
      cli_help: Default::default(),
      cli_long_help: Default::default(),
      cli_visible_aliases: Default::default(),
      cli_long_flag_aliases: Default::default(),
      cli_aliases: Default::default(),
      cli_short_flag_aliases: Default::default(),
      cli_output_formats: Default::default(),
      cli_force_output_format: Default::default(),
      cli_no_output: Default::default(),
      config: Default::default(),
    }
  }
}

pub type Emap = HashMap<String, EpNode>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpNode {
  pub endpoint: Vec<Endpoint>,
  pub route: Emap,
}

// lazy_static! {
//   static ref ENDPOINT: Arc<Mutex<Emap>> = {
//     println!("*******************");
//     let m = HashMap::new();
//     Arc::new(Mutex::new(m))
//   };
// }

const TMP: &str = "/tmp/crud_api_endpoints.json";

fn endpoint_filename() -> String {
  format!("{}-{}", TMP, std::process::id())
}

#[derive(Default, Serialize, Deserialize)]
struct TmpStore {
  ep: Emap,
  inputs: HashMap<String, ApiInputSerde>,
}

fn load_store() -> TmpStore {
  match File::open(endpoint_filename()) {
    Ok(file) => {
      let reader = BufReader::new(file);
      let u: TmpStore =
        serde_json::from_reader(reader).expect("Error reading /tmp/crud_api_endpoints.json.");
      u
    }
    Err(_) => TmpStore::default(),
  }
}

pub fn input_map() -> HashMap<String, ApiInputSerde> {
  load_store().inputs
}

pub fn store_input(input: String, field: impl Into<ApiInputSerde>) {
  let mut store = load_store();
  // let mut prefixes = store.inputs.get(&input).unwrap_or(&vec![]).to_owned();
  // prefixes.push(prefix);
  store.inputs.insert(input, field.into());
  let file = File::create(endpoint_filename()).expect("Can't open file in write mode");
  serde_json::to_writer_pretty(file, &store).unwrap();
}

pub fn endpoints() -> Emap {
  load_store().ep
}

pub fn store_endpoint(epoint: Endpoint) {
  // OK. That's the best piece of code I ever produce.
  //  let map: Emap = endpoints();
  let mut store = load_store();
  let mut segments: Vec<&str> = epoint.cli_route.split('/').collect();
  segments.reverse();

  let map = insert_endpoint(store.ep, &epoint, segments);

  let file = File::create(endpoint_filename()).expect("Can't open file in write mode");
  store.ep = map;
  serde_json::to_writer_pretty(file, &store).unwrap();
}

fn insert_endpoint(map: Emap, ep: &Endpoint, mut segments: Vec<&str>) -> Emap {
  if let Some(segment) = segments.pop() {
    if segment.is_empty() {
      return insert_endpoint(map, ep, segments);
    }
    let mut map = map;
    if segments.is_empty() {
      // We find the leaf
      if let Some(node) = map.get(segment) {
        let mut node = node.to_owned();
        node.endpoint.push(ep.to_owned());
        map.insert(segment.to_string(), node);
      } else {
        let node = EpNode {
          endpoint: vec![ep.to_owned()],
          route: HashMap::new(),
        };
        map.insert(segment.to_string(), node);
      }
      map
    } else if let Some(node) = map.get(segment) {
      let mut node = node.to_owned();
      node.route = insert_endpoint(node.route.to_owned(), ep, segments);
      map.insert(segment.to_string(), node);
      map
    } else {
      let node = EpNode {
        endpoint: vec![],
        route: insert_endpoint(HashMap::new(), ep, segments),
      };
      map.insert(segment.to_string(), node);
      map
    }
  } else {
    map
  }
}

pub fn cleanup() {
  std::fs::remove_file(endpoint_filename()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
  use super::{insert_endpoint, EndpointBuilder};
  use std::collections::HashMap;

  #[test]
  fn test_insert_simple_endpoint() {
    let ep = EndpointBuilder::default()
      .cli_route("/")
      .route("/")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect::<Vec<&str>>();
    segments.reverse();
    let map = HashMap::new();
    let result = insert_endpoint(map, &ep, segments);
    assert_eq!(serde_json::to_string(&result).unwrap(), "{}");
  }

  #[test]
  fn test_insert_one_endpoint_at_one_level_endpoint() {
    let ep = EndpointBuilder::default()
      .cli_route("/post")
      .route("/post")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = HashMap::new();
    let result = insert_endpoint(map, &ep, segments);
    assert_eq!(serde_json::to_string(&result).unwrap(), "{\"post\":{\"endpoint\":[{\"route\":\"/post\",\"method\":\"GET\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]}],\"route\":{}}}");
  }

  #[test]
  fn test_insert_two_endpoints_at_one_level() {
    let ep = EndpointBuilder::default()
      .cli_route("/post")
      .route("/post")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = HashMap::new();
    let map = insert_endpoint(map, &ep, segments);

    let ep = EndpointBuilder::default()
      .cli_route("/post")
      .route("/post")
      .method("POST")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let result = insert_endpoint(map, &ep, segments);

    assert_eq!(serde_json::to_string(&result).unwrap(), "{\"post\":{\"endpoint\":[{\"route\":\"/post\",\"method\":\"GET\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]},{\"route\":\"/post\",\"method\":\"POST\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]}],\"route\":{}}}");
  }

  #[test]
  fn test_insert_three_endpoints_at_two_levels() {
    let map = HashMap::new();
    let ep = EndpointBuilder::default()
      .cli_route("/post")
      .route("/post")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = insert_endpoint(map, &ep, segments);

    let ep = EndpointBuilder::default()
      .cli_route("/post")
      .route("/post")
      .method("POST")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = insert_endpoint(map, &ep, segments);

    let ep = EndpointBuilder::default()
      .cli_route("/post/user")
      .route("/post/user")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = insert_endpoint(map, &ep, segments);
    assert_eq!(serde_json::to_string(&map).unwrap(),"{\"post\":{\"endpoint\":[{\"route\":\"/post\",\"method\":\"GET\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]},{\"route\":\"/post\",\"method\":\"POST\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]}],\"route\":{\"user\":{\"endpoint\":[{\"route\":\"/post/user\",\"method\":\"GET\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post/user\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]}],\"route\":{}}}}}");
  }

  #[test]
  fn test_insert_one_endpoints_at_three_levels() {
    let map = HashMap::new();
    let ep = EndpointBuilder::default()
      .cli_route("/post/comments/replies")
      .route("/post")
      .build()
      .unwrap();
    let mut segments: Vec<&str> = ep.cli_route.split('/').collect();
    segments.reverse();
    let map = insert_endpoint(map, &ep, segments);

    assert_eq!(serde_json::to_string(&map).unwrap(), "{\"post\":{\"endpoint\":[],\"route\":{\"comments\":{\"endpoint\":[],\"route\":{\"replies\":{\"endpoint\":[{\"route\":\"/post\",\"method\":\"GET\",\"result_ok_status\":\"OK\",\"result_ko_status\":[],\"result_multiple\":false,\"result_is_stream\":false,\"cli_route\":\"/post/comments/replies\",\"cli_no_output\":false,\"cli_force_output_format\":false,\"config\":[]}],\"route\":{}}}}}}}");
  }

  #[test]
  fn test_endpoint_default() {
    let ep = EndpointBuilder::default().build().unwrap();
    assert_eq!(ep.route, "".to_string());
    assert_eq!(ep.method, "GET".to_string());
  }
  #[test]
  fn test_endpoint_result_struct() {
    let ep = EndpointBuilder::default()
      .result_struct("Endpoint")
      .build()
      .unwrap();
    assert_eq!(ep.route, "".to_string());
    assert_eq!(ep.method, "GET".to_string());
    assert_eq!(ep.result_struct, "Endpoint".to_string());
  }
}
