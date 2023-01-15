//! # Crud Api Derive
//!
//! Derive implementation for `Crud`.
//!
//! See [`crud`](../crud) crate.

mod crud;
use crud::crud_expension;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::DeriveInput;

#[proc_macro_derive(Crud, attributes(crud))]
#[proc_macro_error]
#[rustfmt::skip::macros(quote)]
pub fn api_run_macro_derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  crud_expension(&ast).into()
}
