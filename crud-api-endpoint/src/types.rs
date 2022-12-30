use darling::FromMeta;
use serde::{Deserialize, Serialize};

/// Wrapper for Vec<String> to implements FromMeta
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VecStringWrapper {
  pub v: Vec<String>,
  pub c: Vec<char>, // also stores the first char of each string
}

impl From<Vec<String>> for VecStringWrapper {
  fn from(v: Vec<String>) -> Self {
    VecStringWrapper {
      c: v
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>(),
      v,
    }
  }
}
impl From<VecStringWrapper> for Vec<String> {
  fn from(val: VecStringWrapper) -> Self {
    val.v
  }
}

impl FromMeta for VecStringWrapper {
  fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
    (match *item {
      syn::NestedMeta::Lit(ref lit) => Self::from_value(lit),
      syn::NestedMeta::Meta(ref mi) => Self::from_meta(mi),
    })
    .map_err(|e| e.with_span(item))
  }

  fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
    (match *item {
      syn::Meta::Path(_) => Self::from_word(),
      syn::Meta::List(ref value) => Self::from_list(
        &value
          .nested
          .iter()
          .cloned()
          .collect::<Vec<syn::NestedMeta>>()[..],
      ),
      syn::Meta::NameValue(ref value) => Self::from_value(&value.lit),
    })
    .map_err(|e| e.with_span(item))
  }

  fn from_none() -> Option<Self> {
    None
  }

  fn from_word() -> darling::Result<Self> {
    Err(darling::Error::unsupported_format("word"))
  }

  fn from_list(_items: &[syn::NestedMeta]) -> darling::Result<Self> {
    Err(darling::Error::unsupported_format("list"))
  }

  fn from_value(value: &syn::Lit) -> darling::Result<Self> {
    (match *value {
      syn::Lit::Bool(ref b) => Self::from_bool(b.value),
      syn::Lit::Str(ref s) => Self::from_string(&s.value()),
      syn::Lit::Char(ref ch) => Self::from_char(ch.value()),
      _ => Err(darling::Error::unexpected_lit_type(value)),
    })
    .map_err(|e| e.with_span(value))
  }

  fn from_char(_value: char) -> darling::Result<Self> {
    Err(darling::Error::unexpected_type("char"))
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    Ok(VecStringWrapper {
      v: value
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
      c: value
        .split(',')
        .map(|s| s.chars().next().unwrap_or_default())
        .collect::<Vec<char>>(),
    })
  }

  fn from_bool(_value: bool) -> darling::Result<Self> {
    Err(darling::Error::unexpected_type("bool"))
  }
}
