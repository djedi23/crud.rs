use super::{enums::ApiInputVariant, ApiInput, ApiInputField};
use crud_api_endpoint::{ApiInputFieldSerde, ApiInputSerde, ApiInputVariantSerde, DataSerde};
use darling::ast::{Data, Fields, Style::Struct};
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, Type};

// Converters into and from ApiInput / crud-api-endpoints::ApiInputSerde

impl From<ApiInput> for ApiInputSerde {
  fn from(val: ApiInput) -> Self {
    ApiInputSerde {
      ident: val.ident.to_string(),
      no_input_file: val.no_input_file,
      heading: val.heading,
      prefix: val.prefix,
      config: val.config,
      data: match val.data {
        Data::Enum(e) => DataSerde::Enum(
          e.into_iter()
            .map(|v| ApiInputVariantSerde {
              ident: v.ident.to_string(),
              fields: v
                .fields
                .fields
                .into_iter()
                .map(|ty| quote! {#ty}.to_string())
                .collect::<Vec<String>>(),
              long: v.long,
              short: v.short,
              no_short: v.no_short,
              no_long: v.no_long,
              heading: v.heading,
              help: v.help,
              long_help: v.long_help,
            })
            .collect::<Vec<ApiInputVariantSerde>>(),
        ),
        Data::Struct(s) => DataSerde::Struct(
          s.fields
            .into_iter()
            .map(|f| ApiInputFieldSerde {
              ident: f.ident.map(|i| i.to_string()),
              ty: {
                let ty = f.ty;
                quote! {#ty}.to_string()
              },
              long: f.long,
              short: f.short,
              no_short: f.no_short,
              heading: f.heading,
              help: f.help,
              long_help: f.long_help,
              possible_values: f.possible_values.map(|pv| pv.into()),
              required: f.required,
              num_args: f.num_args,
            })
            .collect::<Vec<ApiInputFieldSerde>>(),
        ),
      },
    }
  }
}

impl From<ApiInputSerde> for ApiInput {
  fn from(ai: ApiInputSerde) -> Self {
    ApiInput {
      ident: Ident::new(&ai.ident, Span::call_site()),
      data: match ai.data {
        DataSerde::Enum(e) => Data::Enum(
          e.into_iter()
            .map(|v| ApiInputVariant {
              ident: Ident::new(&v.ident, Span::call_site()),
              fields: Fields::new(
                Struct,
                v.fields
                  .into_iter()
                  .map(|s| {
                    let t: Type = syn::parse_str(&s).unwrap();
                    t
                  })
                  .collect::<Vec<Type>>(),
              ),
              long: v.long,
              short: v.short,
              no_short: v.no_short,
              no_long: v.no_long,
              heading: v.heading,
              help: v.help,
              long_help: v.long_help,
            })
            .collect::<Vec<ApiInputVariant>>(),
        ),
        DataSerde::Struct(s) => Data::Struct(Fields::new(
          Struct,
          s.into_iter()
            .map(|f| ApiInputField {
              ident: f.ident.map(|i| Ident::new(&i, Span::call_site())),
              ty: {
                let t: Type = syn::parse_str(&f.ty).unwrap();
                t
              },
              long: f.long,
              short: f.short,
              no_short: f.no_short,
              heading: f.heading,
              help: f.help,
              long_help: f.long_help,
              possible_values: f.possible_values.map(|pv| pv.into()),
              required: f.required,
              num_args: f.num_args,
            })
            .collect::<Vec<ApiInputField>>(),
        )),
      },
      no_input_file: ai.no_input_file,
      heading: ai.heading,
      prefix: ai.prefix,
      config: ai.config,
    }
  }
}
