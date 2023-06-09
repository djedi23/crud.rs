use crate::VecStringWrapper;
use darling::FromMeta;
use proc_macro_error::abort;
use serde::{Deserialize, Serialize};
/// Arguments configuration.
/// We want to avoid argumnent clash. The proposed solution is to reconfigure the standard argmunents.
use std::collections::HashMap;

#[derive(Debug, Default, FromMeta, Clone, Serialize, Deserialize)]
#[darling(default)]
pub struct ApiInputConfig {
  pub arg_name: Option<String>,
  pub ty: Option<String>,
  pub long: Option<String>,
  pub short: Option<char>,
  pub no_short: Option<bool>,
  pub heading: Option<String>,
  pub help: Option<String>,
  pub long_help: Option<String>,
  pub possible_values: Option<VecStringWrapper>,
  /// Force the requirement of this field else use the Option to determine id this field is required or not.
  pub required: Option<bool>,
  /// By default `num_args` is set automatically. You can override the automatism with this arg.
  pub num_args: Option<String>,
}

lazy_static! {
  static ref CONFIGMAP: HashMap<String, ApiInputConfig> = {
    let mut m = HashMap::new();
    m.insert(
      "output_file".into(),
      ApiInputConfig {
        arg_name: Some("output_file".into()),
        ty: Some("String".into()),
        long: Some("output".into()),
        short:Some( 'o'),
        help:Some( "Output file. (default: stdout)".into()),
        long_help: Some( "Output file to save the result in. (default: stdout)".into()),
        heading: Some("Options".into()),
        required:Some(false),
        ..Default::default()
      },
    );
    m.insert(
      "input_file".into(),
      ApiInputConfig {
        arg_name: Some("input_file".into()),
        ty: Some("String".into()),
        long: Some("input".into()),
        short: Some('i'),
        help: Some("Read the data from file ('-' for stdin)".into()),
        long_help: Some("Read the data from a JSON file ('-' for stdin)".into()),
        heading: Some("Options".into()),
        possible_values: None,
        required:Some(false),
        ..Default::default()
      },
    );
    m.insert(
      "input_template".into(),
      ApiInputConfig {
        arg_name: Some("input_template".into()),
        ty: Some("Option<bool>".into()),
        num_args:None,
        long: Some("template".into()),
        short: Some('t'),
        help: Some("Generate an input template".into()),
        long_help: Some("Generate an input template to use with the --input option".into()),
        heading: Some("Options".into()),
        possible_values: None,
        required:Some(false),
        ..Default::default()
      },
    );
    m.insert(
      "output_format".into(),
      ApiInputConfig {
        arg_name: Some( "output_format".into()),
        long: Some("format".into()),
        short: Some('f'),
        // help: "Display result as JSON".into(),
           heading: Some("Formatting".into()),
        // possible_values: vec![],
        ..Default::default()
      },
    );
      m
  };
}

pub fn arg_config(k: &str, local_config: &[ApiInputConfig]) -> ApiInputConfig {
  let global = CONFIGMAP.get(k);
  let local = local_config
    .iter()
    .find(|&c| k == c.arg_name.as_ref().unwrap());

  match (global, local) {
    (None, None) => abort!(k, format!("Can't find '{k}' configuration")),
    (None, Some(l)) => l.to_owned(),
    (Some(g), None) => g.to_owned(),
    (Some(g), Some(l)) => ApiInputConfig {
      arg_name: l.arg_name.to_owned().or_else(|| g.arg_name.to_owned()),
      ty: l.ty.to_owned().or_else(|| g.ty.to_owned()),
      long: l.long.to_owned().or_else(|| g.long.to_owned()),
      short: l.short.to_owned().or_else(|| g.short.to_owned()),
      no_short: l.no_short.to_owned().or_else(|| g.no_short.to_owned()),
      heading: l.heading.to_owned().or_else(|| g.heading.to_owned()),
      help: l.help.to_owned().or_else(|| g.help.to_owned()),
      long_help: l.long_help.to_owned().or_else(|| g.long_help.to_owned()),
      possible_values: l
        .possible_values
        .to_owned()
        .or_else(|| g.possible_values.to_owned()),
      required: l.required.to_owned().or_else(|| g.required.to_owned()),
      num_args: l.num_args.to_owned().or_else(|| g.num_args.to_owned()),
    },
  }
}

// #[allow(dead_code)]
// pub(crate) fn arg_config_quote(k: &str) -> TokenStream {
//   let ac = arg_config(k);
//   field_quote(&ac.into(), None)
// }

// pub(crate) fn parse_arg_config(meta: &MetaList) {
//   let parsed_arg = parse_arg_config_internal(meta);
//   let k = parsed_arg.name.to_owned();
//   let mut arg = arg_config(&k);
//   if !parsed_arg.long.is_empty() {
//     arg.long = parsed_arg.long;
//   }
//   if parsed_arg.short != char::default() {
//     arg.short = parsed_arg.short;
//   }
//   if !parsed_arg.help.is_empty() {
//     arg.help = parsed_arg.help;
//   }
//   if !parsed_arg.heading.is_empty() {
//     arg.heading = parsed_arg.heading;
//   }
//   if !parsed_arg.possible_values.is_empty() {
//     arg.possible_values = parsed_arg.possible_values;
//   }
//   CONFIGMAP // .lock().unwrap()
//     .insert(k, arg);
// }

// #[cfg(test)]
// mod tests {
//   use super::arg_config_quote;

//   #[test]
//   fn test_arg_config_quote() {
//     assert_eq!(arg_config_quote("output").to_string(), "Arg :: new (\"output\") . long (\"output\") . short ('o') . help (\"Output file. (default: stdout)\")");
//   }

//   #[test]
//   #[should_panic]
//   fn test_arg_config_quote_not_exist() {
//     arg_config_quote("no_exist");
//   }
// }
