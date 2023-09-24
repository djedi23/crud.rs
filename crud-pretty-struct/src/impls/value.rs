use crate::{coloring, Meta, PrettyPrint};
use miette::Result;
use owo_colors::OwoColorize;
use pad::PadStr;
use serde_json::Value;
use std::fmt::Write;
use unicode_width::UnicodeWidthStr;

impl PrettyPrint for Value {
  fn meta(&self) -> Meta {
    Meta {
      padding: 0,
      separator: None,
      fields: vec![],
    }
  }
  fn pretty(&self, colored: bool, prefix: Option<String>) -> Result<String> {
    let Meta { separator, .. } = self.meta();

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

    let (v, should_color, should_prefix) = match self {
      Value::Null => ("null".to_string(), true, true),
      Value::Bool(b) => (b.to_string(), true, true),
      Value::Number(n) => (n.to_string(), true, true),
      Value::String(s) => (s.to_owned(), true, true),
      Value::Array(vv) => (
        vv.iter().fold(String::new(), |mut output, item| {
          let end = match item {
            Value::Array(_) | Value::Object(_) => "",
            _ => "\n",
          };
          let _ = write!(
            output,
            "{prefix_}{}{end}",
            item
              .pretty(colored, Some(prefix.clone() + "   "))
              .unwrap_or_default()
              .replacen(&(prefix.clone() + "   "), " - ", 1)
          );
          output
        }),
        false,
        false,
      ),
      Value::Object(o) => (
        {
          let padding = 1 + o.keys().map(|k| k.width()).max().unwrap_or_default();

          o.iter()
            .enumerate()
            .map(|(_i, (k, v))| {
              let separator = match v {
                Value::Array(_) | Value::Object(_) => "-->\n",
                _ => separator,
              };
              let end = match v {
                Value::Array(_) | Value::Object(_) => "",
                _ => "\n",
              };
              let v = v
                .pretty(
                  colored,
                  match v {
                    Value::Array(_) | Value::Object(_) => Some(prefix.clone() + "| "),
                    _ => None,
                  },
                )
                .unwrap_or_default();
              if colored {
                let v = coloring(v, &None);
                format!("{prefix_}{}{separator}{v}{end}", k.pad_to_width(padding))
              } else {
                format!("{prefix}{}{separator}{v}{end}", k.pad_to_width(padding))
              }
            })
            .collect()
        },
        false,
        false,
      ),
    };

    let prefix_ = if should_prefix { prefix_ } else { "".into() };

    let pretty_hashmap = if colored && should_color {
      let v = coloring(v, &None);
      format!("{prefix_}{v}")
    } else {
      format!("{prefix_}{v}")
    };

    Ok(pretty_hashmap)
  }
}

#[cfg(test)]
mod tests {
  use crate::PrettyPrint;
  use serde_json::{json, Value};

  #[test]
  fn value_object() {
    let v: Value = json!({});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "".to_string());

    let v: Value = json!({"a":1});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "a = 1\n".to_string());

    let v: Value = json!({"number":1,"bool":true,"string":"string"});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(
      result,
      "bool   = true\nnumber = 1\nstring = string\n".to_string()
    );
    let v: Value = json!({"array":[1, 3, 4]});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "array -->\n|  - 1\n|  - 3\n|  - 4\n".to_string());

    let v: Value = json!({"object":{"aaa":1,"b":2}});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "object -->\n| aaa = 1\n| b   = 2\n".to_string());

    let v: Value = json!({"object":{"aaa":1,"b":{"a":"aaaa","bbb":["a","bb","ccc"]}}});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "object -->\n| aaa = 1\n| b   -->\n| | a   = aaaa\n| | bbb -->\n| | |  - a\n| | |  - bb\n| | |  - ccc\n".to_string());

    let v: Value = json!({"array":[{"a":1,"bb":2},{"b":2}]});
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(
      result,
      "array -->\n|  - a  = 1\n|    bb = 2\n|  - b = 2\n".to_string()
    );
  }

  #[test]
  fn value_object_colored() {
    let v: Value = json!({});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(result, "".to_string());

    let v: Value = json!({"a":1});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(
      result,
      "a = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n"
        .to_string()
    );

    let v: Value = json!({"number":1,"bool":true,"string":"string"});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(
      result,
     "bool   = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97mtrue\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\nnumber = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\nstring = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n".to_string()
    );
    let v: Value = json!({"array":[1, 3, 4]});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(result,  "array -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m4\u{1b}[39m\u{1b}[0m\n\u{1b}[39m\u{1b}[0m".to_string());

    let v: Value = json!({"object":{"aaa":1,"b":2}});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(result, "object -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| \u{1b}[39maaa = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| \u{1b}[39mb   = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[39m\u{1b}[0m".to_string());

    let v: Value = json!({"object":{"aaa":1,"b":{"a":"aaaa","bbb":["a",{"bb":3,"a":6,"ccc":[1,2,3]},["ccc","ddddd"]]}}});
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(result, "object -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| \u{1b}[39maaa = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| \u{1b}[39mb   -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| | \u{1b}[39ma   = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97maaaa\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | \u{1b}[39mbbb -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| | | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97ma\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39ma   = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m6\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | |    \u{1b}[39mbb  = \u{1b}[1m\u{1b}[97m\u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | |    \u{1b}[39mccc -->\n\u{1b}[1m\u{1b}[97m\u{1b}[38;2;80;80;80m| | |    | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | |    | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | |    | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n\u{1b}[39m\u{1b}[0m\u{1b}[38;2;80;80;80m| | | \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97mccc\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m| | |    \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97mddddd\u{1b}[39m\u{1b}[0m\n\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m\u{1b}[39m\u{1b}[0m".to_string());
  }

  #[test]
  fn value_null() {
    let v: Value = json!(null);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "null".to_string());
  }

  #[test]
  fn value_bool() {
    let v: Value = json!(false);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "false".to_string());

    let v: Value = json!(true);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "true".to_string());
  }

  #[test]
  fn value_string() {
    let v: Value = json!("string");
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "string".to_string());
  }

  #[test]
  fn value_number() {
    let v: Value = json!(5);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "5".to_string());

    let v: Value = json!(-3.014);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, "-3.014".to_string());
  }

  #[test]
  fn value_number_colored() {
    let v: Value = json!(-874);
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(
      result,
      "\u{1b}[1m\u{1b}[97m-874\u{1b}[39m\u{1b}[0m".to_string()
    );
  }

  #[test]
  fn value_number_porefix() {
    let v: Value = json!(842.147854);
    let result = v
      .pretty(false, Some("==> ".to_string()))
      .unwrap_or_default();
    assert_eq!(result, "==> 842.147854".to_string());
  }

  #[test]
  fn value_array() {
    let v: Value = json!([1, 2, 3]);
    let result = v.pretty(false, None).unwrap_or_default();
    assert_eq!(result, " - 1\n - 2\n - 3\n".to_string());
  }

  #[test]
  fn value_array_color() {
    let v: Value = json!([1, 2, 3]);
    let result = v.pretty(true, None).unwrap_or_default();
    assert_eq!(result,"\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n".to_string());
  }

  #[test]
  fn value_array_prefix() {
    let v: Value = json!([1, 2, 3]);
    let result = v
      .pretty(false, Some(">>> ".to_string()))
      .unwrap_or_default();
    assert_eq!(result, ">>>  - 1\n>>>  - 2\n>>>  - 3\n".to_string());
  }

  #[test]
  fn value_array_prefix_colored() {
    let v: Value = json!([1, 2, 3]);
    let result = v.pretty(true, Some(">>> ".to_string())).unwrap_or_default();
    assert_eq!(result, "\u{1b}[38;2;80;80;80m>>> \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m>>> \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\n\u{1b}[38;2;80;80;80m>>> \u{1b}[39m\u{1b}[38;2;80;80;80m - \u{1b}[39m\u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n".to_string());
  }
}
