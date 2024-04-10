use crate::{coloring, Meta, PrettyPrint};
use miette::Result;
use owo_colors::OwoColorize;
use pad::PadStr;

impl<K: std::fmt::Display + PadStr, V: std::fmt::Display> PrettyPrint
  for std::collections::HashMap<K, V>
{
  fn meta(&self) -> Meta {
    Meta {
      padding: 28,
      separator: None,
      fields: vec![],
    }
  }
  fn pretty(&self, colored: bool, prefix: Option<String>, _profile: Option<&str>) -> Result<String> {
    let Meta {
      separator, padding, ..
    } = self.meta();

    let separator = separator.unwrap_or("= ");
    let prefix_ = if let Some(prefix) = &prefix {
      if colored {
        prefix.truecolor(80, 80, 80).to_string()
      } else {
        prefix.to_owned()
      }
    } else {
      "".into()
    };
    let prefix = &prefix.unwrap_or_default();

    dbg!(prefix);
    let pretty_hashmap: String = self
      .iter()
      .map(|(k, v)| {
        if colored {
          let v = coloring(v.to_string(), &None);
          format!("{prefix_}{}{separator}{v}\n", k.pad_to_width(padding))
        } else {
          format!("{prefix}{}{separator}{v}\n", k.pad_to_width(padding))
        }
      })
      .collect();

    Ok(pretty_hashmap)
  }
}
