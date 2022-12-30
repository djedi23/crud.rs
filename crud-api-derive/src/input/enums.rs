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
  pub no_short: bool,
  pub heading: Option<String>,
  pub help: Option<String>,
  pub long_help: Option<String>,
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_enum_decl(
  _prefix: Option<String>,
  variants: &[ApiInputVariant],
  heading: Option<String>,
  conflict_input_file: TokenStream,
) -> Vec<TokenStream> {
  let variants_names: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();
  let variants_args = variants
    .iter()
    .map(|variant| {
      let arg = variant_decl_quote(variant, heading.to_owned());
      let name = variant.ident.to_string();
      let mut conflicts = variants_names.clone();
      conflicts.retain(|v| name != *v);
      let conflicts_with_all = if conflicts.is_empty() {
        quote!()
      } else {
        quote!(.conflicts_with_all(&[#(#conflicts),*]))
      };
      // let prefix = match &prefix {
      //   Some(prefix) => format!("{}-{}", prefix, input.ident),
      //   None => input.ident.to_string(),
      // };
      let variant_fields = variant
        .fields
        .iter()
        .map(|f| {
          if let Type::Path(f) = f {
            let type_ident = &f.path.segments.first().unwrap().ident;
            quote!(let app =
		     <#type_ident> :: clap(app,
					  Some(crud_api::ApiInputOptions{
					      conflicts_with_all:vec![#(#conflicts.into()),*]
					  }));
	    )
          } else {
            quote!()
          }
        })
        .collect::<Vec<TokenStream>>();
      let arg = quote! {
	  let app=app.arg(#arg
			.conflicts_with_all(&conflicts)
			#conflicts_with_all
			#conflict_input_file);
	  #(#variant_fields)*
      };
      arg
    })
    .collect::<Vec<TokenStream>>();
  variants_args
}

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

      quote! {if matches.contains_id(#name){
	    #ident :: #variant_ident #variant_parameters
	} }
    })
    .collect::<Vec<TokenStream>>();

  let match_variants = quote! {
      #(#variants_values ) else*
      else { #ident :: default() }
  };

  match_variants
}

#[rustfmt::skip::macros(quote)]
fn variant_decl_quote(variant: &ApiInputVariant, heading: Option<String>) -> TokenStream {
  let name = variant.ident.to_string();
  let long = {
    let l = variant.long.as_ref().unwrap_or(&name).to_lowercase();
    quote! {.long(#l)}
  };
  let short = if variant.no_short {
    quote!{}
  } else {
    let short = if let Some(short) = variant.short {
      short
    } else {
      let long = variant.long.as_ref().unwrap_or(&name).to_lowercase();
      long.chars().next().unwrap()
    };
    quote!{.short(#short)}
  };
  let help = if let Some(h) = &variant.help {
    quote! {.help(#h)}
  } else {
    quote! {}
  };
  let long_help = if let Some(h) = &variant.long_help {
    quote! {.long_help(#h)}
  } else {
    quote! {}
  };
  let heading = if let Some(h) = &variant.heading {
    quote! {.help_heading(#h)}
  } else {
    let h = heading.unwrap_or_else(|| "Payload".to_string());
    quote! {.help_heading(#h)}
  };
  // let possible_values = if let Some(pv) = &variant.possible_values {
  //   let pv = &pv.v;
  //   quote! {.possible_values([#(#pv),*])}
  // } else {
  //   quote! {}
  // };
  quote! {
      clap::Arg::new(#name).action(clap::ArgAction::Set)
	  #long #short #help #long_help #heading
  }
}
