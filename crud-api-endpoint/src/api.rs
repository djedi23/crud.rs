use crate::Endpoint;
use darling::{
  ast::{Data, Fields},
  FromDeriveInput, FromField, FromMeta, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(api))]
pub struct ApiField {
  pub ident: Option<Ident>,
  pub ty: Type,
  /// the field won't appears when display as the table
  #[darling(default)]
  pub table_skip: bool,
  /// Format of the field
  pub table_format: Option<FieldFormat>,
}

#[derive(Debug, FromMeta, Clone)]
pub enum FieldFormat {
  /// Format a string as datetime
  Date {
    /// Format of the date.
    ///
    /// The format is specified here:
    /// <https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers>
    ///
    /// # Example
    ///
    /// ```sh
    /// #[api(format(date(format = "%F %T")))]
    /// ```
    format: String,
  },
}

#[derive(Debug, FromVariant)]
#[darling(attributes(api))]
#[allow(dead_code)]
pub struct ApiVariant {
  ident: Ident,
  //  discriminant: Option<syn::Expr>,
  fields: darling::ast::Fields<Type>,
  //  attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(api, suggest), forward_attrs(derive))]
pub struct Api {
  pub ident: Ident,
  pub data: Data<ApiVariant, ApiField>,
  pub attrs: Vec<syn::Attribute>,

  #[darling(default)]
  #[darling(multiple)]
  pub endpoint: Vec<Endpoint>,

  /// returns a stream of bytes for this struct
  #[darling(rename = "stream")]
  #[darling(default)]
  pub result_is_stream: bool,
}

#[rustfmt::skip::macros(quote)]
pub fn table_impl<T: Into<ApiField> + Clone>(
  struct_ident: &Ident,
  data: &Data<ApiVariant, T>,
  is_pretty: bool,
) -> TokenStream {
  let (headers, table_convertions) = match data {
    Data::Enum(_) => {
      todo!("Générer les entetes et les données pour chaque variant");
      //      (vec![], quote!())
    }
    Data::Struct(Fields { fields, .. }) => {
      let headers: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter(|field| {
          let field: ApiField = (*field).clone().into();
          !field.table_skip
        })
        .map(|field| {
          let field: ApiField = (*field).clone().into();
          let f = field
            .ident
            .as_ref()
            .expect("Field without ident")
            .to_string();
          quote! {#f.to_string()}
        })
        .collect();

      let table_convertions: proc_macro2::TokenStream = fields
        .iter()
        .filter(|field| {
          let field: ApiField = (*field).clone().into();
          !field.table_skip
        })
        .map(|field| {
          let field: ApiField = (*field).clone().into();
          let fname = field.ident.as_ref().expect("Field without ident");
          // I'm hardcoding std::Display
          let unformated_value = if is_option_vec(&field.ty) {
            // WARNING: Not all Vec<T> have the join method !
            quote!{self.#fname .as_ref().unwrap() .join(", ").replace('\n', "\\n")}
          } else if is_option(&field.ty) {
            quote! {self.#fname .clone().unwrap_or_default().to_string().replace('\n', "\\n")}
          } else if is_vec(&field.ty) {
            quote!{self.#fname .join(", ").replace('\n', "\\n")}
          } else {
            quote! {self.#fname .to_string().replace('\n', "\\n")}
          };

          let formated_value = if let Some(FieldFormat::Date { format }) = field.table_format {
            quote!(#unformated_value.parse::<chrono::DateTime<chrono::Utc>>()
		    .into_diagnostic().wrap_err("Can't parse Date")?
		    .format(#format).to_string()
	    )
          } else {
            unformated_value
          };

          quote!(#formated_value ,)
        })
        .collect();
      (headers, table_convertions)
    }
  };

  let to_output = if is_pretty {
    quote!{
	  fn to_output(&self) -> miette::Result<String>
	  where Self:crud_pretty_struct::PrettyPrint
	{
	    use is_terminal::IsTerminal;
	    self.pretty(std::io::stdout().is_terminal(), None, None)
	  }
      }
  } else {
    quote!{
	fn to_output(&self) -> miette::Result<String>
	where Self: Serialize
	{
	    use miette::IntoDiagnostic;
	    serde_yaml::to_string(self).into_diagnostic()
	}
      }
  };

  quote! {impl crud_api::Api for #struct_ident {
      fn to_table_header(&self) -> Vec<String> {
	  vec![#(#headers),*]
      }
      fn to_table(&self) -> miette::Result<Vec<String>> {
	  Ok(vec![#table_convertions])
      }
      #to_output
  }}
}

fn is_option(ty: &Type) -> bool {
  if let Type::Path(s) = ty {
    if let Some(x) = s.path.segments.first() {
      return x.ident.eq("Option");
    }
  }
  false
}
fn is_vec(ty: &Type) -> bool {
  if let Type::Path(s) = ty {
    if let Some(x) = s.path.segments.first() {
      return x.ident.eq("Vec");
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
