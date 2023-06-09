use super::enums::{derive_enum_command_match, derive_enum_decl_command};
use crate::input::ApiInput;
use crud_api_endpoint::{input_map, VecStringWrapper};
use darling::{ast::Fields, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

#[derive(Clone, Debug, FromField)]
#[darling(attributes(api))]
pub struct ApiInputField {
  pub ident: Option<Ident>,
  pub ty: Type,
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

#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_struct_decl(
  prefix: Option<String>,
  fields: &Fields<ApiInputField>,
  heading: Option<String>,
  conflict_input_file: TokenStream,
) -> Vec<TokenStream> {
  let inputs = input_map();
  let fields_args = fields
    .fields
    .iter()
    .map(|f| {
      if let Some(input_from_type) = inputs.get(
        &{
          let ty = strip_type(&f.ty);
          quote!(#ty)
        }
        .to_string(),
      ) {
        let input: ApiInput = input_from_type.clone().into();
        let prefix = match &prefix {
          Some(prefix) => format!("{}-{}", prefix, f.ident.as_ref().unwrap()),
          None => f.ident.clone().unwrap().to_string(),
        };
        match &input.data {
          darling::ast::Data::Struct(fields) => derive_struct_decl(
            Some(prefix),
            fields,
            heading.to_owned(),
            conflict_input_file.to_owned(),
          ),
          darling::ast::Data::Enum(variants) => derive_enum_decl_command(Some(prefix), variants),
        }
        .into_iter()
        .collect::<TokenStream>()
      } else {
        let arg = field_quote(f, heading.to_owned(), prefix.to_owned());
        quote! {let app=app.arg(#arg .conflicts_with_all(&conflicts) #conflict_input_file);}
      }
    })
    .collect::<Vec<TokenStream>>();
  fields_args
}
#[rustfmt::skip::macros(quote)]
pub(crate) fn derive_struct_match(
  ident: &Ident,
  prefix: Option<String>,
  fields: &Fields<ApiInputField>,
) -> TokenStream {
  let inputs = input_map();
  let fields_values = fields
    .fields
    .iter()
    .map(|f| {
      if let Some(input_from_type) = inputs.get(
        &{
          let ty = strip_type(&f.ty);
          quote!(#ty)
        }
        .to_string(),
      ) {
        let fname = f.ident.as_ref().unwrap();
        let input: ApiInput = input_from_type.clone().into();
        let prefix = Some(match &prefix {
          Some(prefix) => format!("{}-{}", prefix, f.ident.as_ref().unwrap()),
          None => f.ident.clone().unwrap().to_string(),
        });
        let struct_quote = match &input.data {
          darling::ast::Data::Struct(fields) => derive_struct_match(&input.ident, prefix, fields),
          darling::ast::Data::Enum(variants) => {
            derive_enum_command_match(&input.ident, prefix, variants)
          }
        }
        .into_iter()
        .collect::<TokenStream>();
        if is_option(&f.ty) {
          quote!{#fname: Some(#struct_quote),}
        } else {
          quote!{#fname: #struct_quote,}
        }
      } else {
        let fv = field_matched_value(f, prefix.to_owned());
        quote! {#fv ,}
      }
    })
    .collect::<TokenStream>();

  let struct_from_clap = quote! {
      #ident {
	  #fields_values
      }};
  struct_from_clap
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn field_quote(
  field: &ApiInputField,
  heading: Option<String>,
  prefix: Option<String>,
) -> TokenStream {
  let raw_name = field.ident.as_ref().unwrap().to_string();
  let ty = strip_type(&field.ty);

  let is_bool = quote!(#ty).to_string().eq("bool");
  let arg_action = if is_vec(&field.ty) || is_option_vec(&field.ty) {
    quote!(clap::ArgAction::Append)
  } else if is_bool && is_option(&field.ty) {
    quote!(clap::ArgAction::SetTrue)
  } else {
    quote!(clap::ArgAction::Set)
  };
  let long = {
    let l = field.long.as_ref().unwrap_or(&raw_name);
    let l = if let Some(prefix) = &prefix {
      format!("{prefix}-{l}").to_lowercase()
    } else {
      l.to_string()
    };
    quote! {.long(#l)}
  };
  let short = match field.no_short {
    Some(true) => {
      quote!{}
    }
    _ => {
      let short = if let Some(short) = field.short {
        short
      } else {
        let long = field.long.as_ref().unwrap_or(&raw_name);
        long.chars().next().unwrap()
      };
      quote!{.short(#short)}
    }
  };
  let help = if let Some(h) = &field.help {
    let h = if is_option(&field.ty) {
      format!("(option) {h}")
    } else {
      h.to_string()
    };
    quote! {.help(#h)}
  } else {
    quote! {}
  };
  let long_help = if let Some(h) = &field.long_help {
    let h = if is_option(&field.ty) {
      format!("(option) {h}")
    } else {
      h.to_string()
    };
    quote! {.long_help(#h)}
  } else {
    quote! {}
  };
  let heading = if let Some(h) = &field.heading {
    quote! {.help_heading(#h)}
  } else {
    let h = heading.unwrap_or_else(|| "Payload".to_string());
    quote! {.help_heading(#h)}
  };
  let value_parser = if let Some(pv) = &field.possible_values {
    let pv = &pv.v;
    quote!(clap::builder::PossibleValuesParser::new([#(#pv),*]))
  } else {
    quote! {clap::value_parser!(#ty)}
  };
  let required = if let Some(required) = &field.required {
    *required
  } else {
    !is_option(&field.ty)
  };
  let num_args = if let Some(num_args) = &field.num_args {
    quote!(.num_args(#num_args))
  } else if is_vec(&field.ty) || is_option_vec(&field.ty) || is_bool {
    quote!()
  } else {
    quote!(.num_args(clap::builder::ValueRange::SINGLE))
  };

  let name = if let Some(prefix) = prefix {
    format!("{prefix}-{raw_name}").to_lowercase()
  } else {
    raw_name
  };

  quote! {
      clap::Arg::new(#name)
	  .value_parser(#value_parser)
          .action(#arg_action)
	  .required(#required)
	  #long #short #help #long_help #heading #num_args
  }
}
#[rustfmt::skip::macros(quote)]
fn field_matched_value(field: &ApiInputField, prefix: Option<String>) -> TokenStream {
  let name = field.ident.as_ref().unwrap();
  let ty = strip_type(&field.ty);
  let sname = if let Some(prefix) = prefix {
    format!("{prefix}-{name}").to_lowercase()
  } else {
    name.to_string()
  };

  if is_option_vec(&field.ty) {
    quote!(#name : matches
           .get_many::<#ty>(#sname)
	   .map(|vals| vals.cloned()
		.collect::<Vec<#ty>>()))
  } else if is_vec(&field.ty) {
    quote!(#name : matches
            .get_many::<#ty>(#sname)
            .unwrap()
            .cloned()
            .collect::<Vec<#ty>>())
  } else {
    let value = quote!(matches.get_one::<#ty>(#sname).cloned());
    if is_option(&field.ty) {
      if quote!(#ty).to_string().eq("bool") {
        quote!(#name : if let Some(false) = #value {None} else { #value })
      } else {
        quote!(#name : #value)
      }
    } else {
      quote!(#name : #value.unwrap())
    }
  }
}

fn is_vec(ty: &Type) -> bool {
  if let Type::Path(s) = ty {
    if let Some(x) = s.path.segments.first() {
      return x.ident.eq("Vec");
    }
  }
  false
}

fn is_option(ty: &Type) -> bool {
  if let Type::Path(s) = ty {
    if let Some(x) = s.path.segments.first() {
      return x.ident.eq("Option");
    }
  }
  false
}

fn is_option_vec(ty: &Type) -> bool {
  // It a copy of strip_type without the recursivity
  fn strip_type_no_rec(ty: &Type) -> &Type {
    if is_option(ty) || is_vec(ty) {
      if let Type::Path(s) = ty {
        if let Some(segment) = s.path.segments.first() {
          if let PathArguments::AngleBracketed(first_arg) = &segment.arguments {
            if let GenericArgument::Type(result_type) = first_arg.args.first().unwrap() {
              return result_type;
            }
          }
        }
      }
    }
    ty
  }

  if is_option(ty) {
    is_vec(strip_type_no_rec(ty))
  } else {
    false
  }
}

/// Return a type without Option or Vec:
/// T -> T
/// Option<T> -> T
/// Vec<T> -> T
fn strip_type(ty: &Type) -> &Type {
  if is_option(ty) || is_vec(ty) {
    if let Type::Path(s) = ty {
      if let Some(segment) = s.path.segments.first() {
        if let PathArguments::AngleBracketed(first_arg) = &segment.arguments {
          if let GenericArgument::Type(result_type) = first_arg.args.first().unwrap() {
            return strip_type(result_type);
          }
        }
      }
    }
  }
  ty
}

#[cfg(test)]
mod tests {
  use crate::input::structs::{is_option_vec, is_vec, strip_type};

  use super::is_option;
  use syn::{parse_str, Type};

  #[test]
  fn is_option_test() {
    let ty: Type = parse_str("String").unwrap();
    assert!(!is_option(&ty));
    let ty: Type = parse_str("Option<String>").unwrap();
    assert!(is_option(&ty));
    let ty: Type = parse_str("Vec<String>").unwrap();
    assert!(!is_option(&ty));
  }

  #[test]
  fn is_vec_test() {
    let ty: Type = parse_str("String").unwrap();
    assert!(!is_vec(&ty));
    let ty: Type = parse_str("Option<String>").unwrap();
    assert!(!is_vec(&ty));
    let ty: Type = parse_str("Vec<String>").unwrap();
    assert!(is_vec(&ty));
  }

  #[test]
  fn is_option_vec_test() {
    let ty: Type = parse_str("String").unwrap();
    assert!(!is_option_vec(&ty));
    let ty: Type = parse_str("Option<String>").unwrap();
    assert!(!is_option_vec(&ty));
    let ty: Type = parse_str("Vec<String>").unwrap();
    assert!(!is_option_vec(&ty));
    let ty: Type = parse_str("Option<Vec<String>>").unwrap();
    assert!(is_option_vec(&ty));
  }

  #[test]
  fn type_strip_test() {
    let ty_string: Type = parse_str("String").unwrap();
    assert_eq!(strip_type(&ty_string), &ty_string);
    let ty: Type = parse_str("Option<String>").unwrap();
    assert_eq!(strip_type(&ty), &ty_string);
    let ty: Type = parse_str("Vec<String>").unwrap();
    assert_eq!(strip_type(&ty), &ty_string);
    let ty: Type = parse_str("Option<Vec<String>>").unwrap();
    assert_eq!(strip_type(&ty), &ty_string);
  }
}
