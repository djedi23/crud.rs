use crate::{
  input::{field_quote, ApiInputField},
  ApiRun,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse_str;

#[rustfmt::skip::macros(quote)]
pub(crate) fn settings(api: &ApiRun) -> proc_macro2::TokenStream {
  let settings_name = if let Some(name) = &api.infos.name {
    let name_s = name.to_string();
    quote! {#name_s}
  } else {
    quote! {clap::crate_name!()}
  };
  let settings_organisation = if let Some(organisation) = &api.infos.organisation {
    let organisation_s = organisation.to_string();
    quote! {#organisation_s}
  } else {
    quote! {""}
  };
  let settings_qualifier = if let Some(qualifier) = &api.infos.qualifier {
    let qualifier_s = qualifier.to_string();
    quote! {#qualifier_s}
  } else {
    quote! {""}
  };
  let settings_env_prefix = if let Some(env_prefix) = &api.infos.env_prefix {
    let env_prefix_s = env_prefix.to_string();
    quote! {#env_prefix_s}
  } else {
    quote! {"APP"}
  };

  quote! {
  let settings = crud_api::settings::settings(#settings_qualifier,
      #settings_organisation,
      #settings_name,
      #settings_env_prefix)?;

  }
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn init_clap(api: &ApiRun) -> proc_macro2::TokenStream {
  let app_name = if let Some(name) = &api.infos.name {
    let name_s = name.to_string();
    quote! {commands = commands.name(#name_s);}
  } else {
    quote! {}
  };
  let app_author = if let Some(author) = &api.infos.author {
    let author_s = author.to_string();
    quote! {commands = commands.author(#author_s);}
  } else {
    quote! {
    commands = commands.author(clap::crate_authors!("\n"));}
  };
  let app_version = if let Some(version) = &api.infos.version {
    let version_s = version.to_string();
    quote! {commands = commands.version(#version_s);}
  } else {
    quote! {
    commands = commands.version(clap::crate_version!());}
  };
  let app_about = if let Some(about) = &api.infos.about {
    let about_s = about.to_string();
    quote! {commands = commands.about(#about_s);}
  } else {
    quote! {
    commands = commands.author(clap::crate_description!());}
  };

  let profile = setting_clap_decl("profile", "profile", "Profile to use. default: no profile");
  let arg_base = setting_clap_decl("base_url", "base-url", "Override the base url");
  quote! {
      let mut commands = crud_api::cli::init_clap();
      #app_name
      #app_author
      #app_version
      #app_about
      #profile
      #arg_base
  }
}

fn setting_clap_decl(ident: &str, long: &str, help: &str) -> TokenStream {
  let base = field_quote(
    &ApiInputField {
      ident: Some(Ident::new(ident, Span::call_site())),
      ty: parse_str("String").unwrap(),
      long: Some(long.to_string()),
      short: None,
      no_short: Some(true),
      heading: Some("Configuration".to_string()),
      help: Some(help.to_string()),
      long_help: None,
      possible_values: None,
      required: Some(false),
      num_args: None,
    },
    None,
    None,
  );
  let arg_base = quote! {commands = commands.arg(#base);};
  arg_base
}
