use crate::ApiInputConfig;
use serde::{Deserialize, Serialize};

// Serializable ApiInput.
// From and Into are in crud-api-derive/input.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInputSerde {
  pub ident: String,
  pub data: DataSerde,
  pub no_input_file: bool,
  pub heading: Option<String>,
  pub prefix: Option<String>,
  pub config: Vec<ApiInputConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSerde {
  Enum(Vec<ApiInputVariantSerde>),
  Struct(Vec<ApiInputFieldSerde>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInputFieldSerde {
  pub ident: Option<String>,
  pub ty: String,
  pub long: Option<String>,
  pub short: Option<char>,
  pub no_short: Option<bool>,
  pub heading: Option<String>,
  pub help: Option<String>,
  pub long_help: Option<String>,
  pub possible_values: Option<Vec<String>>,
  pub required: Option<bool>,
  pub num_args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInputVariantSerde {
  pub ident: String,
  pub fields: Vec<String>,
  pub long: Option<String>,
  pub short: Option<char>,
  pub no_short: bool,
  pub heading: Option<String>,
  pub help: Option<String>,
  pub long_help: Option<String>,
}
