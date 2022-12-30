use crud_api_endpoint::{store_endpoint, table_impl, Api};
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::DeriveInput;

pub fn api(ast: &DeriveInput) -> TokenStream {
  let api = Api::from_derive_input(ast).unwrap();
  for endpoint in api.endpoint {
    let mut endpoint = endpoint;
    if endpoint.result_struct.is_empty() {
      endpoint.result_struct = api.ident.to_string();
    }
    endpoint.result_is_stream = endpoint.result_is_stream || api.result_is_stream;
    store_endpoint(endpoint);
  }

  table_impl(&api.ident, &api.data).into()
}
