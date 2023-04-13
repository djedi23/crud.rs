//! # Api Derive
//!
//! Derive implementation for `Api`.
//!
//! See [`crud-api`](../crud-api) crate.

mod api;
mod config;
mod gen_clap_declarations;
mod gen_clap_matches;
mod gen_init;
mod input;

use api::api;
use crud_api_endpoint::ApiRun;
use darling::FromDeriveInput;
use gen_clap_declarations::subcommands;
use gen_clap_matches::argmatches;
use gen_init::{init_clap, settings};
use input::api_input_derive;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse, DeriveInput};

/// Attribute used by `ApiRun`. [struct@ApiRun]
#[proc_macro_derive(ApiRun, attributes(api))]
#[proc_macro_error]
#[rustfmt::skip::macros(quote)]
#[allow(clippy::let_and_return)]
pub fn api_run_macro_derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse(input).unwrap();
  let api = ApiRun::from_derive_input(&ast).unwrap();
  let name = &api.ident;

  let settings = settings(&api);
  let init_clap = init_clap(&api);
  let subcommands = subcommands();
  let base_url = &api.infos.base_url;
  let matches = argmatches();

  let eh: Vec<proc_macro2::TokenStream> = api
    .extra_header
    .iter()
    .map(|h| {
      let key = &h.key;
      let value = &h.value;
      quote!(crud_api::http::Header{key:#key, value:#value})
    })
    .collect();

  let out = quote! {
      impl #name {
	 async fn run() -> miette::Result<()> {
	     pretty_env_logger::init();
	     let mut auth = Auth::default();
	     let extra_headers: Vec<crud_api::http::Header> = vec![#(#eh),*];
	     #settings
	     #init_clap
	     commands = auth.clap_auth(commands);
	     #subcommands

	     let matches = crud_api::cli::get_matches(&commands)?;
	     let base_url = if let Ok(url) =
		 crud_api::settings::get_settings(&settings, &matches, "base_url") {
		     url
	     } else {
		 #base_url.to_string()
	     };
	     auth.clap_matches(&matches,&mut commands,&settings);
	     match matches.subcommand() {
		  #matches
		  Some(("completion", completions)) =>
		      crud_api::completions::generate_completions(completions, &mut commands),
		  Some((_,_))=> commands.print_help().into_diagnostic()?,
		  None => commands.print_help().into_diagnostic()?,
	      }
	      Ok(())
	  }
      }
  }
  .into();
  #[cfg(feature = "dump-derives")]
  println!("{}", out);
  out
}

#[proc_macro_derive(Api, attributes(api))]
#[proc_macro_error]
#[allow(clippy::let_and_return)]
pub fn api_macro_derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse(input).unwrap();
  let out = api(&ast);
  #[cfg(feature = "dump-derives")]
  println!("{}", out);
  out
}

#[proc_macro_derive(ApiInput, attributes(api))]
#[proc_macro_error]
#[allow(clippy::let_and_return)]
pub fn api_input_macro_derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse(input).unwrap();
  let out = api_input_derive(&ast).into();
  #[cfg(feature = "dump-derives")]
  println!("{}", out);
  out
}
