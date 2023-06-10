use case::CaseExt;
use darling::{ast::Fields, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(api))]
pub struct ApiInputVariant {
  pub ident: Ident,
  pub fields: Fields<Type>,
  pub long: Option<String>,
  pub short: Option<char>,
  #[darling(default)]
  pub no_long: bool,
  #[darling(default)]
  pub no_short: bool,
  pub heading: Option<String>,
  pub help: Option<String>,
  pub long_help: Option<String>,
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_enum_decl_command(
  _prefix: Option<String>,
  variants: &[ApiInputVariant],
) -> Vec<TokenStream> {
  let variants_commands = variants
    .iter()
    .map(|variant| {
      let arg = variant_command_decl_quote(variant);
      let variant_fields = variant
        .fields
        .iter()
        .map(|f| {
          if let Type::Path(f) = f {
            let type_ident = &f.path.segments.first().unwrap().ident;
            quote!(let arg = <#type_ident> :: clap(arg,None);)
          } else {
            quote!()
          }
        })
        .collect::<Vec<TokenStream>>();
      let arg = quote! {
	    let arg = #arg;
	    #(#variant_fields)*
	    let app=app.subcommand(arg);
      };
      arg
    })
    .collect::<Vec<TokenStream>>();
  variants_commands
}

#[rustfmt::skip::macros(quote)]
fn variant_command_decl_quote(variant: &ApiInputVariant) -> TokenStream {
  let name = variant.ident.to_string();
  let command_name = name.to_snake().to_dashed();
  let long = if variant.no_long {
    quote!()
  } else {
    let l = variant.long.as_ref().unwrap_or(&name).to_lowercase();
    quote! {.long_flag(#l)}
  };
  let short = if variant.no_short {
    quote!{}
  } else {
    let short = if let Some(short) = variant.short {
      short
    } else {
      let short = variant.long.as_ref().unwrap_or(&name).to_lowercase();
      short.chars().next().unwrap()
    };
    quote!{.short_flag(#short)}
  };
  let about = if let Some(h) = &variant.help {
    quote! {.about(#h)}
  } else {
    quote! {}
  };
  let long_about = if let Some(h) = &variant.long_help {
    quote! {.long_about(#h)}
  } else {
    quote! {}
  };
  quote! {
      clap::Command::new(#command_name)
	  #long #short
      #about #long_about
  }
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_enum_command_match(
  ident: &Ident,
  _prefix: Option<String>,
  variants: &[ApiInputVariant],
) -> TokenStream {
  let variants_values = variants
    .iter()
    .map(|variant| {
      let variant_ident = &variant.ident;
      let name = variant.ident.to_string();
      let command_name = name.to_snake().to_dashed();

      let default_value = variant
        .fields
        .iter()
        .map(|f| {
          if let Type::Path(f) = f {
            let type_ident = &f.path.segments.first().unwrap().ident;
            quote!( <#type_ident> :: from_clap_matches(matches)? )
          } else {
            quote!()
          }
        })
        .collect::<Vec<TokenStream>>();

      let variant_parameters = if default_value.is_empty() {
        quote!()
      } else {
        quote!( (#(#default_value),*))
      };

      quote! {Some((#command_name, matches)) => {
	    #ident :: #variant_ident #variant_parameters
      } }
    })
    .collect::<Vec<TokenStream>>();

  let match_variants = quote! {
	match matches.subcommand() {
	    #(#variants_values )*
	    Some((&_, _)) =>{#ident :: default()}
	    None => {#ident :: default()}
	}
  };

  match_variants
}

////////////////////////////////////////////////////////////

#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_enum_match(
  ident: &Ident,
  _prefix: Option<String>,
  variants: &[ApiInputVariant],
) -> TokenStream {
  let variants_values = variants
    .iter()
    .map(|variant| {
      let variant_ident = &variant.ident;
      let name = variant.ident.to_string();
      let command_name = name.to_snake().to_dashed();

      quote! {Some(#command_name) => {
	    #ident :: #variant_ident
      } }
    })
    .collect::<Vec<TokenStream>>();

  let match_variants = quote! {
	match matches.get_one::<String>("level").cloned().as_deref() {
	    #(#variants_values )*
	    Some( _) =>{#ident :: default()}
	    None => {#ident :: default()}
	}
  };

  match_variants
}
