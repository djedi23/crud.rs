use crud_api_endpoint::{store_endpoint, table_impl, Api};
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, MetaList};

#[rustfmt::skip::macros(quote)]
pub fn api(ast: &DeriveInput) -> TokenStream {
  let api = Api::from_derive_input(ast).unwrap();
  let is_pretty = api.attrs.iter().any(|Attribute { meta, .. }| match meta {
    syn::Meta::List(MetaList { tokens, .. }) => tokens
      .clone()
      .into_iter()
      .any(|ident| ident.to_string() == "PrettyPrint"),
    _ => false,
  });

  for endpoint in api.endpoint {
    let mut endpoint = endpoint;
    if endpoint.result_struct.is_empty() {
      endpoint.result_struct = api.ident.to_string();
    }
    endpoint.result_is_stream = endpoint.result_is_stream || api.result_is_stream;
    store_endpoint(endpoint);
  }

  let ident = api.ident;
  let table = table_impl(&ident, &api.data, is_pretty);
  quote! {
  #table
      impl TryFrom<crud_api::DummyTryFrom> for #ident {
	  type Error = String;
	  fn try_from(_value: crud_api::DummyTryFrom) -> std::result::Result<Self, Self::Error> {
	      Err(String::new())
	  }
      }
  }
  .into()
}
