mod enums;
mod serde;
mod structs;

pub(crate) use self::structs::{field_quote, ApiInputField};
use self::{
  enums::{derive_enum_decl, derive_enum_match, ApiInputVariant},
  structs::{derive_struct_decl, derive_struct_match},
};
use crud_api_endpoint::{arg_config, ApiInputConfig};
use darling::{ast::Data, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(api))]
pub struct ApiInput {
  pub ident: Ident,
  pub data: Data<ApiInputVariant, ApiInputField>,
  /// Disable the possibility to read the input from file or stdin.
  #[darling(default)]
  pub no_input_file: bool,
  pub heading: Option<String>,
  pub prefix: Option<String>,
  #[darling(default, multiple)]
  pub config: Vec<ApiInputConfig>,
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn api_input_derive(ast: &DeriveInput) -> TokenStream {
  let input = ApiInput::from_derive_input(ast).unwrap();
  let ident = input.to_owned().ident;
  crud_api_endpoint::store_input(ident.to_string(), input.to_owned());
  let conflict_input_file = if input.no_input_file {
    quote!{}
  } else {
    quote!{.conflicts_with_all(["input_file","input_template"])}
  };

  let fields_args = match &input.data {
    Data::Struct(fields) => derive_struct_decl(
      input.prefix.to_owned(),
      fields,
      input.heading,
      conflict_input_file,
    ),
    Data::Enum(variants) => derive_enum_decl(
      input.prefix.to_owned(),
      variants,
      input.heading,
      conflict_input_file,
    ),
  };

  let struct_from_clap = match &input.data {
    Data::Struct(fields) => derive_struct_match(&ident, input.prefix, fields),
    Data::Enum(variants) => derive_enum_match(&ident, input.prefix, variants),
  };

  let app_maybe_wrapped_to_read_from_file = if input.no_input_file {
    quote!{}
  } else {
    let field: ApiInputField = arg_config("input_file", &input.config).into();
    let input_arg = field_quote(&field, None, None);
    let field: ApiInputField = arg_config("input_template", &input.config).into();
    let template_arg = field_quote(&field, None, None);
    quote!{let app = app.arg(#input_arg);
	   let app = app.arg(#template_arg);
    }
  };
  let get_input_from_file_or_clap = if input.no_input_file {
    quote!{#struct_from_clap}
  } else {
    let get_input_template = quote!{crud_api::clap_match_template::<#ident>(matches)?};
    quote!{
	if let Some(payload) = crud_api::clap_match_input_from_file(matches)? {
	    payload
	} else if #get_input_template {
	    Self::default()
	} else {
	    #struct_from_clap
	}
    }
  };

  let out = quote! {
  impl ::crud_api::ApiInput for #ident {
      fn clap(app: clap::Command,
	     options: Option<crud_api::ApiInputOptions>) -> clap::Command {
	  let conflicts = if let Some(options) = options {
	      options.conflicts_with_all
	  } else {
	      vec![]
	  };

	  #app_maybe_wrapped_to_read_from_file
	  #(#fields_args)*
	  app
      }
      fn from_clap_matches(matches: &clap::ArgMatches) -> miette::Result<Self>{
	  Ok(#get_input_from_file_or_clap)
      }
  }
};

  out
}
