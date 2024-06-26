mod types;

use darling::{ast::Data, FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use strum::Display;
use syn::{parse, DeriveInput, Expr, GenericArgument, PathArguments, Type};
use types::VecStringWrapper;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(pretty))]
struct PrettyStruct {
  ident: Ident,
  data: Data<PrettyVariant, PrettyField>,
  /// Separator between the label and the value. Default: "= "
  separator_glyph: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Display)]
enum Color {
  Black,
  Blue,
  Cyan,
  Green,
  Magenta,
  Red,
  White,
  Yellow,
}

#[derive(Debug, FromField)]
#[darling(attributes(pretty))]
struct PrettyField {
  ident: Option<Ident>,
  ty: Type,
  /// This field is formatted with `PrettyPrint`
  #[darling(default)]
  is_pretty: bool,
  /// Set the displayed label
  label: Option<String>,
  /// Value color.
  color: Option<Color>,
  /// Specific label when color is on.
  label_color: Option<Color>,
  /// Skip this field. Don't display it.
  #[darling(default)]
  skip: bool,
  /// Don't display this field if `None`.
  #[darling(default)]
  skip_none: bool,
  /// Field Formatter.
  /// Accept a closure or a function name.
  /// The signature of the closure is `Fn(value:&dyn ToString, colored:bool) -> (value:String, colored_value:bool)`.
  ///
  /// Examples:
  /// - Direct closure:
  /// ```ignore
  /// #[pretty(formatter = |x,_|(format!("{} format",x.to_string()),false))]
  /// ```
  /// - Function name:
  /// ```ignore
  /// fn boobool(b: &dyn ToString,_:bool) -> (String,bool) {
  ///     (if b.to_string() == *"true" {
  ///   "✔"
  ///     } else {
  ///   "✘"
  ///     }
  ///      .to_string(),
  ///      true)
  /// }
  /// // ...
  /// #[pretty(formatter = boobool)]
  /// ```
  ///
  formatter: Option<Expr>,

  /// Print this field only when one of this profile is active or profiles is none.
  /// Profiles is a coma separated string.
  profiles: Option<VecStringWrapper>,
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(pretty))]
struct PrettyVariant {
  ident: Ident,
  //  discriminant: Option<syn::Expr>,
  fields: darling::ast::Fields<Type>,
  //  attrs: Vec<syn::Attribute>,
  /// Value color.
  color: Option<Color>,
  /// Ddisplay this label instead of the variant name. You can use it only on Unit variants.
  label: Option<String>,
}

/// The struct can be pretty printed.
///
/// ```ignore
/// use crud_pretty_struct_derive::PrettyPrint;
/// #[derive(PrettyPrint)]
/// struct Foo {
///     #[pretty(color="green")]
///     a: u32,
///     #[pretty(skip_none)]
///     b: Option<String>,
///     #[pretty(formatter=crud_pretty_struct::formatters::bool_check_formatter)]
///     c: bool,
///     #[pretty(is_pretty)]
///     d: OtherPrettyStruct
/// }
/// ```
#[proc_macro_derive(PrettyPrint, attributes(pretty))]
#[proc_macro_error]
#[rustfmt::skip::macros(quote)]
#[allow(clippy::let_and_return)]
pub fn pretty_struct_derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse(input).unwrap();
  let pretty = PrettyStruct::from_derive_input(&ast).unwrap();
  //  dbg!(&pretty.data);

  let out = match &pretty.data {
    Data::Enum(_) => derive_enum(pretty),
    Data::Struct(_) => derive_struct(pretty),
  };
  #[cfg(feature = "dump-derives")]
  println!("{}", out);
  out
}

fn derive_struct(pretty: PrettyStruct) -> TokenStream {
  let pretty_ident = pretty.ident;
  let glyph = pretty.separator_glyph.unwrap_or_else(|| "= ".to_string());

  let field_names: Vec<String> = match &pretty.data {
    Data::Enum(_) => unreachable!(),
    Data::Struct(strct) => strct
      .fields
      .iter()
      .filter(|&f| !f.skip)
      .map(|field| {
        field
          .label
          .to_owned()
          .unwrap_or(field.ident.as_ref().unwrap().to_string())
      })
      .collect(),
  };

  let profiles: Vec<Vec<String>> = match &pretty.data {
    Data::Enum(_) => unreachable!(),
    Data::Struct(strct) => strct
      .fields
      .iter()
      .filter(|&f| !f.skip)
      .map(|field| field.profiles.to_owned().unwrap_or(vec![].into()).into())
      .collect(),
  };
  let profiles: Vec<proc_macro2::TokenStream> = profiles
    .iter()
    .map(|profile| quote!(#(#profile),*))
    .collect();

  let colors: Vec<proc_macro2::TokenStream> = match &pretty.data {
    Data::Enum(_) => unreachable!(),
    Data::Struct(strct) => strct
      .fields
      .iter()
      //    .filter(|&f| !f.skip)
      .map(|field| match &field.color {
        Some(color) => {
          let color = Ident::new(&color.to_string(), Span::call_site());
          quote!(Some(crud_pretty_struct::Color::#color))
        }
        None => quote!(None),
      })
      .collect(),
  };

  let label_colors: Vec<proc_macro2::TokenStream> = match &pretty.data {
    Data::Enum(_) => unreachable!(),
    Data::Struct(strct) => strct
      .fields
      .iter()
      .filter(|&f| !f.skip)
      .map(|field| match &field.label_color {
        Some(color) => {
          let color = Ident::new(&color.to_string(), Span::call_site());
          quote!(Some(crud_pretty_struct::Color::#color))
        }
        None => quote!(None),
      })
      .collect(),
  };

  let values_expr: Vec<proc_macro2::TokenStream> = match pretty.data {
    Data::Enum(_) => unreachable!(),
    Data::Struct(strct) => strct
      .fields
      .into_iter()
      .filter(|f| !f.skip)
      .map(|field| {
        let id: Ident = (*field.ident.as_ref().unwrap()).clone();
        if field.is_pretty {
          if is_option_vec(&field.ty) {
            let skip_none = field.skip_none;
            quote!(crud_pretty_struct::MetaValue::OptionVecPretty{value:self.#id.as_ref().map(
			    |vec| vec.iter().map(|x| x as &dyn PrettyPrint).collect()),skip_none:#skip_none})
          } else if is_option(&field.ty) {
            let skip_none = field.skip_none;
            quote!(crud_pretty_struct::MetaValue::OptionPretty{value:self.#id.as_ref().map(
			    |x| x as &dyn PrettyPrint),skip_none:#skip_none})
          } else if is_vec(&field.ty) {
            quote!(crud_pretty_struct::MetaValue::VecPretty(self.#id.iter().map(
			    |x| x as &dyn PrettyPrint).collect()))
          } else {
            quote!(crud_pretty_struct::MetaValue::Pretty(&self.#id))
          }
        } else if is_option_vec(&field.ty) {
          let skip_none = field.skip_none;
          quote!(crud_pretty_struct::MetaValue::OptionVecString{value:self.#id.as_ref().map(
			|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
									skip_none:#skip_none})
        } else if is_option(&field.ty) {
          let formatter = match field.formatter {
            None => quote!(None),
            Some(expr) => match expr {
              Expr::Closure(_) | Expr::Path(_) => quote!(Some(&#expr)),
              _ => abort_call_site!("Closure expected but received \"{}\".", quote!(#expr)),
            },
          };
          let skip_none = field.skip_none;
          quote!(crud_pretty_struct::MetaValue::OptionString{value:self.#id.as_ref().map(
			|x| x as &dyn ToString), formatter:#formatter, skip_none:#skip_none})
        } else if is_vec(&field.ty) {
          quote!(crud_pretty_struct::MetaValue::VecString(self.#id.iter().map(
			|x| x as &dyn ToString).collect()))
        } else {
          let formatter = match field.formatter {
            None => quote!(None),
            Some(expr) => match expr {
              Expr::Closure(_) | Expr::Path(_) => quote!(Some(&#expr)),
              _ => abort_call_site!("Closure expected but received \"{}\".", quote!(#expr)),
            },
          };
          quote!(crud_pretty_struct::MetaValue::String{value:&self.#id,formatter:#formatter})
        }
      })
      .collect(),
  };

  let padding = 1
    + &field_names
      .iter()
      .map(|n| n.width())
      .max()
      .unwrap_or_default();

  quote!(impl crud_pretty_struct::PrettyPrint for #pretty_ident {
  fn meta(&self) ->crud_pretty_struct::Meta {
      crud_pretty_struct::Meta {
    padding: #padding,
    separator: Some(#glyph),
    fields: vec![
        #(
      crud_pretty_struct::MetaField {
          profiles: vec![#profiles],
          field_prefix: crud_pretty_struct::FieldPrefix::Label {
        label: #field_names,
        label_color: #label_colors,
          },
          color: #colors,
          value: #values_expr,
      }
        ),*
    ]
      }
  }
    })
  .into()
}

fn derive_enum(pretty: PrettyStruct) -> TokenStream {
  let pretty_ident = pretty.ident;

  let colors: Vec<proc_macro2::TokenStream> = match &pretty.data {
    Data::Enum(enm) => vec![{
      let enum_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|_variant| pretty_ident.clone())
        .collect();
      let variant_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|variant| variant.ident)
        .collect();
      let variant_param: Vec<proc_macro2::TokenStream> = enm
        .clone()
        .into_iter()
        .map(|variant| match variant.fields.style {
          darling::ast::Style::Unit => quote!(),
          darling::ast::Style::Tuple => quote! {(value)}, // TODO: itérer sur tous les champs
          darling::ast::Style::Struct => unimplemented!(),
        })
        .collect();

      let variant_color: Vec<proc_macro2::TokenStream> = enm
        .iter()
        .map(|variant| match &variant.color {
          Some(color) => {
            let color = Ident::new(&color.to_string(), Span::call_site());
            quote!(Some(crud_pretty_struct::Color::#color))
          }
          None => quote!(None),
        })
        .collect();

      quote!(match self {
      #(#enum_ident::#variant_ident #variant_param => #variant_color),*
              }
        )
    }],
    Data::Struct(_) => unreachable!(),
  };

  let field_prefix: Vec<proc_macro2::TokenStream> = match &pretty.data {
    Data::Enum(enm) => vec![{
      let enum_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|_variant| pretty_ident.clone())
        .collect();
      let variant_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|variant| variant.ident)
        .collect();
      let variant_param: Vec<proc_macro2::TokenStream> = enm
        .clone()
        .into_iter()
        .map(|variant| match variant.fields.style {
          darling::ast::Style::Unit => quote!(),
          darling::ast::Style::Tuple => quote! {(value)}, // TODO: itérer sur tous les champs
          darling::ast::Style::Struct => unimplemented!(),
        })
        .collect();

      let variant_prefix: Vec<proc_macro2::TokenStream> = enm
        .iter()
        .map(|variant| match variant.fields.style {
          darling::ast::Style::Unit => {
            quote!(crud_pretty_struct::FieldPrefix::None)
          }
          darling::ast::Style::Tuple => {
            quote! {crud_pretty_struct::FieldPrefix::Multiline}
          } // TODO: itérer sur tous les champs
          darling::ast::Style::Struct => unimplemented!(),
        })
        .collect();

      quote!(match self {
      #(#enum_ident::#variant_ident #variant_param => #variant_prefix),*
              }
        )
    }],
    Data::Struct(_) => unreachable!(),
  };

  let values_expr: Vec<proc_macro2::TokenStream> = match pretty.data {
    Data::Enum(enm) => vec![{
      let enum_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|_variant| pretty_ident.clone())
        .collect();
      let variant_ident: Vec<Ident> = enm
        .clone()
        .into_iter()
        .map(|variant| variant.ident)
        .collect();
      let variant_param: Vec<proc_macro2::TokenStream> = enm
        .clone()
        .into_iter()
        .map(|variant| match variant.fields.style {
          darling::ast::Style::Unit => quote!(),
          darling::ast::Style::Tuple => quote! {(value)}, // TODO: itérer sur tous les champs
          darling::ast::Style::Struct => unimplemented!(),
        })
        .collect();

      let variant_value: Vec<proc_macro2::TokenStream> = enm
        .into_iter()
        .map(|variant| match variant.fields.style {
          darling::ast::Style::Unit => {
            let value_str = if let Some(label) = variant.label {
              label
            } else {
              variant.ident.to_string()
            };
            quote!(crud_pretty_struct::MetaValue::Variant{value:&#value_str, formatter:None})
          }
          darling::ast::Style::Tuple => quote! {crud_pretty_struct::MetaValue::Pretty(value)}, // TODO: itérer sur tous les champs
          darling::ast::Style::Struct => unimplemented!(),
        })
        .collect();
      quote!(match self {
      #(#enum_ident::#variant_ident #variant_param => #variant_value),*
              })
    }],
    Data::Struct(_) => unreachable!(),
  };

  quote!(impl crud_pretty_struct::PrettyPrint for #pretty_ident {
  fn meta(&self) ->crud_pretty_struct::Meta {
      crud_pretty_struct::Meta {
    padding: 0,
    separator: None,
    fields: vec![
        #(
      crud_pretty_struct:: MetaField {
          profiles: Vec::new(), // No profiles in enums
          field_prefix: #field_prefix,
          color: #colors,
          value: #values_expr,
      }
        ),*
    ]
      }
  }
    })
  .into()
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

// /// Return a type without Option or Vec:
// /// T -> T
// /// Option<T> -> T
// /// Vec<T> -> T
// fn strip_type(ty: &Type) -> &Type {
//   if is_option(ty) || is_vec(ty) {
//     if let Type::Path(s) = ty {
//       if let Some(segment) = s.path.segments.first() {
//         if let PathArguments::AngleBracketed(first_arg) = &segment.arguments {
//           if let GenericArgument::Type(result_type) = first_arg.args.first().unwrap() {
//             return strip_type(result_type);
//           }
//         }
//       }
//     }
//   }
//   ty
// }
