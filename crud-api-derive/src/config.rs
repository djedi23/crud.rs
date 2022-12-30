/// Arguments configuration.
/// We want to avoid argumnent clash. The proposed solution is to reconfigure the standard argmunents.
use crate::input::ApiInputField;
use crud_api_endpoint::ApiInputConfig;
use proc_macro2::Span;

impl From<ApiInputConfig> for ApiInputField {
  fn from(c: ApiInputConfig) -> Self {
    ApiInputField {
      ident: c.arg_name.map(|i| syn::Ident::new(&i, Span::call_site())),
      ty: syn::parse_str(&c.ty.unwrap()).expect("Can't parse type in ApiInputConfig"),
      long: c.long,
      short: c.short,
      no_short: c.no_short,
      heading: c.heading,
      help: c.help,
      long_help: c.long_help,
      possible_values: c.possible_values,
      required: c.required,
      num_args: c.num_args,
    }
  }
}
