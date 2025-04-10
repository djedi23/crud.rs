//! # Pretty Struct
//!
//! Displays (json) structures and enums in a pretty way.
//!
//! This crate is linked to the crud library. If I have time and motivation to generalize it, it can be an indenpendant crate.
//!
//! ## Example
//!
//! ```rust
//! use crud_pretty_struct::PrettyPrint;
//! # #[derive(PrettyPrint)]
//! # struct OtherPrettyStruct {val:u32}
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(color="green")]
//!     a: u32,
//!     #[pretty(skip_none)]
//!     b: Option<String>,
//!     #[pretty(formatter=crud_pretty_struct::formatters::bool_check_formatter)]
//!     c: bool,
//!     #[pretty(is_pretty)]
//!     d: OtherPrettyStruct
//! }
//! # let var = Foo{a:0,b:None,c:false,d:OtherPrettyStruct{val:0}};
//! // Instanciate a `var` of type  `Foo`
//! println!("{}",var.pretty(true,None,None).expect("Can prettify var"));
//! ```
//!
//! ## Field Options
//!
//! ##### `is_pretty`
//!
//! the nested struct implements `PrettyPrint` and should be printed using it.
//!
//! ```rust
//! use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct OtherPrettyStruct {}
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(is_pretty)]
//!     field: OtherPrettyStruct
//! }
//! ```
//!
//! ##### `label`
//!
//! custom label for this field
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(label="☀️ my field")]
//!     field: u32
//! }
//! ```
//! ##### `color`
//!
//! custom color for the value of this field. The avaiable colors are [Color].
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(color="red")]
//!     field: u32
//! }
//! ```
//! ##### `label_color`
//!
//! custom color for the label of dthis field. The avaiable colors are [Color].
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(label_color="red")]
//!     field: u32
//! }
//! ```
//! ##### `skip`
//!
//! skip the field. It won't be display.
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(skip)]
//!     field: u32
//! }
//! ```
//! ##### `skip_none`
//!
//! skip the field  if the value is `None`. The type of the field should be an `Option<T>`.
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(skip_none)]
//!     field: Option<u32>
//! }
//! ```
//! ##### `profile`
//! the field is displayed only when this field profiles matched the profile declare when calling the `pretty` function.
//!
//! ```rust
//! # use crud_pretty_struct::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!   #[pretty(profiles = "a")]
//!   field1: u32,
//!   #[pretty(profiles = "a,b")]
//!   field2: bool,
//! }
//!
//! let foo = Foo{field1:0, field2:false};
//! foo.pretty(false, None, Some("b")).unwrap(); //  print only `field2`
//! ```
//! ##### `formatter`
//!
//! Custom value formatter for this field.
//!
//! Some [formatters] are shipped in this crate.
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(formatter=crud_pretty_struct::formatters::bool_check_formatter)]
//!     field: bool
//! }
//! ```
//!
//! Formatters should follow this signature:
//! ```rust
//! type Formatter = dyn Fn(/*value:*/ &dyn ToString, /*colored:*/ bool) -> miette::Result<(String, bool)>;
//! ```
//! Parameters:
//! - `value`: the value to format
//! - `colored`: when `true` the formatted value can be colored
//!
//! Result:
//! - String: the formatted value
//! - bool: returns `true` if the value have colors. returns `false` if the value don't have colors then default color will be applied.
//!
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! struct Foo {
//!     #[pretty(formatter=|x, _| Ok((format!("{} kg", x.to_string()), false)))]
//!     field: f32
//! }
//! ```
//!
//! ## Enum Option
//!
//! Limitations on enums:
//! - unit variants are supported
//! - tuple variants with only 1 argument are supported
//!
//! ##### `color`
//!
//! custom color for this variant avaiable colors are [Color].
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! enum Foo {
//!     #[pretty(color="red")]
//!     Variant
//! }
//! ```
//!
//! ##### `label`
//!
//! custom label for this variant
//! ```rust
//! # use crud_pretty_struct_derive::PrettyPrint;
//! #[derive(PrettyPrint)]
//! enum Foo {
//!     #[pretty(label="☀️ my field")]
//!     Variant
//! }
//! ```
//!
//!
pub mod formatters;
pub mod impls;

use crate::formatters::identity_formatter;
pub use crud_pretty_struct_derive::*;
//pub use impls::*;
use miette::Result;
use owo_colors::OwoColorize;
use pad::PadStr;
use std::fmt::Write;

pub type Formatter = dyn Fn(&dyn ToString, bool) -> Result<(String, bool)>;
pub enum MetaValue<'a> {
  String {
    value: &'a dyn ToString,
    formatter: Option<&'a Formatter>,
  },
  Pretty(&'a dyn PrettyPrint),
  OptionString {
    value: Option<&'a dyn ToString>,
    formatter: Option<&'a Formatter>,
    skip_none: bool,
  },
  OptionPretty {
    value: Option<&'a dyn PrettyPrint>,
    skip_none: bool,
  },
  VecString(Vec<&'a dyn ToString>),
  VecPretty(Vec<&'a dyn PrettyPrint>),
  OptionVecString {
    value: Option<Vec<&'a dyn ToString>>,
    skip_none: bool,
  },
  OptionVecPretty {
    value: Option<Vec<&'a dyn PrettyPrint>>,
    skip_none: bool,
  },
  Variant {
    value: &'a dyn ToString,
    formatter: Option<&'a Formatter>,
  },
}

#[derive(PartialEq)]
pub enum Color {
  Black,
  Blue,
  Cyan,
  Green,
  Magenta,
  Red,
  White,
  Yellow,
}

#[derive(PartialEq)]
pub enum FieldPrefix<'a> {
  Label {
    label: &'a str,
    label_color: Option<Color>,
  },
  Multiline,
  None,
}

pub struct MetaField<'a> {
  pub profiles: Vec<&'a str>,
  pub field_prefix: FieldPrefix<'a>,
  pub color: Option<Color>,
  pub value: MetaValue<'a>,
}

pub struct Meta<'a> {
  pub padding: usize,
  pub separator: Option<&'a str>,
  pub fields: Vec<MetaField<'a>>,
}

fn coloring(value: String, color: &Option<Color>) -> String {
  match color {
    Some(color) => match color {
      Color::Red => value.red().bold().to_string(),
      Color::Black => value.black().bold().to_string(),
      Color::Blue => value.blue().bold().to_string(),
      Color::Cyan => value.cyan().bold().to_string(),
      Color::Green => value.green().bold().to_string(),
      Color::Magenta => value.magenta().bold().to_string(),
      Color::White => value.white().bold().to_string(),
      Color::Yellow => value.yellow().bold().to_string(),
    },
    None => value.bright_white().bold().to_string(),
  }
}

fn label_coloring(label: &str, colored: bool, color: &Option<Color>) -> String {
  if colored {
    match color {
      Some(color) => match color {
        Color::Red => label.red().to_string(),
        Color::Black => label.black().to_string(),
        Color::Blue => label.blue().to_string(),
        Color::Cyan => label.cyan().to_string(),
        Color::Green => label.green().to_string(),
        Color::Magenta => label.magenta().to_string(),
        Color::White => label.white().to_string(),
        Color::Yellow => label.yellow().to_string(),
      },
      None => label.to_string(),
    }
  } else {
    label.to_string()
  }
}

pub trait PrettyPrint {
  fn meta(&self) -> Meta;
  fn pretty(&self, colored: bool, prefix: Option<String>, profile: Option<&str>) -> Result<String> {
    let Meta {
      fields,
      separator,
      padding,
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
    let prefix = prefix.unwrap_or_default();
    fields
      .into_iter()
      .filter(|MetaField { profiles, .. }| {
        if let Some(profile) = &profile {
          profiles.contains(profile)
        } else {
          true
        }
      })
      .map(
        |MetaField {
           field_prefix,
           color,
           value,
           ..
         }| {
          match field_prefix {
            FieldPrefix::None | FieldPrefix::Multiline => {
              match value {
                MetaValue::String { value, formatter } => {
                  let formatter = formatter.unwrap_or(&identity_formatter);
                  let (value, colored_value) = formatter(value, colored)?;
                  Ok(format!(
                    "{prefix_}{}\n",
                    if colored && !colored_value {
                      coloring(value, &color)
                    } else {
                      value
                    }
                  ))
                }
                MetaValue::Variant { value, formatter } => {
                  let formatter = formatter.unwrap_or(&identity_formatter);
                  let (value, colored_value) = formatter(value, colored)?;
                  Ok(format!(
                    "{prefix_}{}\n",
                    if colored && !colored_value {
                      coloring(value, &color)
                    } else {
                      value
                    }
                  ))
                }
                MetaValue::Pretty(value) => Ok(format!(
                  "{prefix_}{}",
                  value.pretty(colored, Some(prefix.clone()), profile)?
                )),
                MetaValue::OptionString {
                  value,
                  formatter,
                  skip_none,
                } => Ok(if value.is_none() && skip_none {
                  String::new()
                } else {
                  match value {
                    Some(value) => {
                      let formatter = formatter.unwrap_or(&identity_formatter);
                      let (value, colored_value) = formatter(value, colored)?;
                      format!(
                        "{prefix_}{}\n",
                        if colored && !colored_value {
                          coloring(value, &color)
                        } else {
                          value
                        }
                      )
                    }
                    None => {
                      format!(
                        "{prefix_}{}\n",
                        if colored {
                          "null".magenta().to_string() // TODO: coloring
                        } else {
                          "null".to_string()
                        }
                      )
                    }
                  }
                }),
                MetaValue::OptionPretty { value, skip_none } => Ok(match value {
                  Some(value) => format!(
                    "{prefix_}{}",
                    value.pretty(colored, Some(prefix.clone() + "| "), profile)?
                  ),
                  None => {
                    if skip_none {
                      String::new()
                    } else {
                      format!(
                        "{prefix_}{}\n",
                        if colored {
                          "null".magenta().to_string() // TODO: coloring
                        } else {
                          "null".to_string()
                        }
                      )
                    }
                  }
                }),
                MetaValue::VecString(vec) => Ok(format!(
                  "{prefix_}{}",
                  vec.iter().fold(String::new(), |mut output, i| {
                    let _ = writeln!(
                      output,
                      " - {}",
                      if colored {
                        coloring(i.to_string(), &color)
                      } else {
                        i.to_string()
                      }
                    );
                    output
                  })
                )),
                MetaValue::VecPretty(vec) => Ok(format!("{prefix_}{}", {
                  vec
                    .iter()
                    .map(|value| {
                      Ok(
                        value
                          .pretty(colored, Some(prefix.clone() + "   "), profile)?
                          .replacen("   ", " - ", 1),
                      )
                    })
                    .collect::<Result<String>>()?
                })),
                MetaValue::OptionVecString { value, skip_none } => {
                  Ok(if value.is_none() && skip_none {
                    String::new()
                  } else {
                    format!(
                      "{prefix_}{}",
                      if colored {
                        match value {
                          Some(vec) => {
                            "\n".to_string()
                              + &vec.iter().fold(String::new(), |mut output, i| {
                                let _ = writeln!(output, " - {}", coloring(i.to_string(), &color));
                                output
                              })
                          }
                          None => " null\n".magenta().to_string(), // TODO: coloring
                        }
                      } else {
                        match value {
                          Some(vec) => {
                            "\n".to_string()
                              + &vec.iter().fold(String::new(), |mut output, i| {
                                let _ = writeln!(output, " - {}", i.to_string());
                                output
                              })
                          }
                          None => " null\n".to_string(),
                        }
                      }
                    )
                  })
                }
                MetaValue::OptionVecPretty { value, skip_none } => {
                  Ok(if value.is_none() && skip_none {
                    String::new()
                  } else {
                    format!(
                      "{prefix_}{}",
                      match value {
                        Some(vec) =>
                          "\n".to_string()
                            + &vec
                              .iter()
                              .map(|i| Ok(
                                i.pretty(colored, Some(prefix.clone() + "   "), profile)?
                                  .replacen("   ", " - ", 1)
                              ))
                              .collect::<Result<String>>()?,
                        None =>
                          if colored {
                            " null\n".magenta().to_string() // TODO: coloring
                          } else {
                            " null\n".to_string()
                          },
                      }
                    )
                  })
                }
              }
            }
            FieldPrefix::Label { label, label_color } => {
              let label = label_coloring(label, colored, &label_color);
              match value {
                MetaValue::String { value, formatter } => {
                  let formatter = formatter.unwrap_or(&identity_formatter);
                  let (value, colored_value) = formatter(value, colored)?;
                  Ok(format!(
                    "{prefix_}{}{separator}{}\n",
                    label.pad_to_width(padding),
                    if colored && !colored_value {
                      coloring(value, &color)
                    } else {
                      value
                    }
                  ))
                }
                MetaValue::Variant { value, formatter } => {
                  let formatter = formatter.unwrap_or(&identity_formatter);
                  let (value, colored_value) = formatter(value, colored)?;
                  Ok(format!(
                    "{prefix_}{}{separator}{}\n",
                    label.pad_to_width(padding),
                    if colored && !colored_value {
                      coloring(value, &color)
                    } else {
                      value
                    }
                  ))
                }
                MetaValue::Pretty(value) => match value.meta().fields.first().unwrap().field_prefix {
                  FieldPrefix::None => Ok(format!(
                    "{prefix_}{}{separator}{}",
                    label.pad_to_width(padding),
                    value.pretty(colored, Some(prefix.clone()), profile)?
                  )),
                  FieldPrefix::Multiline => Ok(format!(
                    "{prefix_}{label} -->\n{}",
                    value
                      .pretty(colored, Some(prefix.clone() + "| "), profile)?
                      .replacen("| ", "", 1)
                  )),
                  _ => Ok(format!(
                    "{prefix_}{label} -->\n{}",
                    value.pretty(colored, Some(prefix.clone() + "| "), profile)?
                  )),
                },
                MetaValue::OptionString {
                  value,
                  formatter,
                  skip_none,
                } => Ok(if value.is_none() && skip_none {
                  String::new()
                } else {
                  match value {
                    Some(value) => {
                      let formatter = formatter.unwrap_or(&identity_formatter);
                      let (value, colored_value) = formatter(value, colored)?;
                      format!(
                        "{prefix_}{}{separator}{}\n",
                        label.pad_to_width(padding),
                        if colored && !colored_value {
                          coloring(value, &color)
                        } else {
                          value
                        }
                      )
                    }
                    None => {
                      format!(
                        "{prefix_}{}{separator}{}\n",
                        label.pad_to_width(padding),
                        if colored {
                          "null".magenta().to_string() // TODO: coloring
                        } else {
                          "null".to_string()
                        }
                      )
                    }
                  }
                }),
                MetaValue::OptionPretty { value, skip_none } => Ok(match value {
                  Some(value) => {
                    if value.meta().fields.first().unwrap().field_prefix == FieldPrefix::None {
                      format!(
                        "{prefix_}{}{separator}{}",
                        label.pad_to_width(padding),
                        value.pretty(colored, Some(prefix.clone()), profile)?
                      )
                    } else {
                      format!(
                        "{prefix_}{label} -->\n{}",
                        value.pretty(colored, Some(prefix.clone() + "| "), profile)?
                      )
                    }
                  }
                  None => {
                    if skip_none {
                      String::new()
                    } else {
                      format!(
                        "{prefix_}{}{separator}{}\n",
                        label.pad_to_width(padding),
                        if colored {
                          "null".magenta().to_string() // TODO: coloring
                        } else {
                          "null".to_string()
                        }
                      )
                    }
                  }
                }),
                MetaValue::VecString(vec) => Ok(format!(
                  "{prefix_}{label} :\n{}",
                  vec.iter().fold(String::new(), |mut output, i| {
                    let _ = writeln!(
                      output,
                      " - {}",
                      if colored {
                        coloring(i.to_string(), &color)
                      } else {
                        i.to_string()
                      }
                    );
                    output
                  })
                )),
                MetaValue::VecPretty(vec) => Ok(format!("{prefix_}{label} :\n{}", {
                  vec
                    .iter()
                    .map(|value| {
                      Ok(
                        value
                          .pretty(colored, Some(prefix.clone() + "   "), profile)?
                          .replacen("   ", " - ", 1),
                      )
                    })
                    .collect::<Result<String>>()?
                })),
                MetaValue::OptionVecString { value, skip_none } => {
                  Ok(if value.is_none() && skip_none {
                    String::new()
                  } else {
                    format!(
                      "{prefix_}{label} :{}",
                      if colored {
                        match value {
                          Some(vec) => {
                            "\n".to_string()
                              + &vec.iter().fold(String::new(), |mut output, i| {
                                let _ = writeln!(output, " - {}", coloring(i.to_string(), &color));
                                output
                              })
                          }
                          None => " null\n".magenta().to_string(), // TODO: coloring
                        }
                      } else {
                        match value {
                          Some(vec) => {
                            "\n".to_string()
                              + &vec.iter().fold(String::new(), |mut output, i| {
                                let _ = writeln!(output, " - {}", i.to_string());
                                output
                              })
                          }
                          None => " null\n".to_string(),
                        }
                      }
                    )
                  })
                }
                MetaValue::OptionVecPretty { value, skip_none } => {
                  Ok(if value.is_none() && skip_none {
                    String::new()
                  } else {
                    format!(
                      "{prefix_}{label} :{}",
                      match value {
                        Some(vec) =>
                          "\n".to_string()
                            + &vec
                              .iter()
                              .map(|i| Ok(
                                i.pretty(colored, Some(prefix.clone() + "   "), profile)?
                                  .replacen("   ", " - ", 1)
                              ))
                              .collect::<Result<String>>()?,
                        None =>
                          if colored {
                            " null\n".magenta().to_string() // TODO: coloring
                          } else {
                            " null\n".to_string()
                          },
                      }
                    )
                  })
                }
              }
            }
          }
        },
      )
      .collect::<Result<String>>()
  }
}

#[cfg(test)]
mod tests {
  use crate::{coloring, Color, FieldPrefix, Meta, MetaField, MetaValue, PrettyPrint};

  #[test]
  fn empty_struct() {
    struct T1 {}
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 1,
          separator: None,
          fields: vec![],
        }
      }
    }

    let s = T1 {};
    assert_eq!(s.pretty(false, None, None).unwrap(), "".to_string());
  }

  #[test]
  fn simple_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    //    print!("{}", s.pretty(false, None,&None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    = 5\nbb   = string\ncccc = false\n".to_string()
    );
  }

  #[test]
  fn struct_with_profile() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec!["a"],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec!["b"],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec!["a", "b"],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    = 5\nbb   = string\ncccc = false\n".to_string()
    );
    assert_eq!(
      s.pretty(false, None, Some("a")).unwrap(),
      "a    = 5\ncccc = false\n".to_string()
    );
    assert_eq!(
      s.pretty(false, None, Some("b")).unwrap(),
      "bb   = string\ncccc = false\n".to_string()
    );
    assert_eq!(s.pretty(false, None, Some("c")).unwrap(), "".to_string());
  }

  #[test]
  fn nested_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: T1,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::Pretty(&self.n),
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      },
    };
    //    print!("{}", s.pretty(false, None,&None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
    );
  }

  #[test]
  fn simple_struct_custom_separator() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: Some("-> "),
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    //    print!("{}", s.pretty(false, None,&None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    -> 5\nbb   -> string\ncccc -> false\n".to_string()
    );
  }

  #[test]
  fn simple_struct_colored() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    //    print!("{}", s.pretty(true, None));
    assert_eq!(
    s.pretty(true,None, None).unwrap(),
    "a    = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nbb   = \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\ncccc = \u{1b}[1m\u{1b}[97mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
  }

  #[test]
  fn simple_struct_custom_color() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: Some(Color::Green),
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: Some(Color::Yellow),
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: Some(Color::Magenta),
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    //    print!("{}", s.pretty(true, None));
    assert_eq!(
    s.pretty(true,None, None).unwrap(),
  "a    = \u{1b}[1m\u{1b}[32m5\u{1b}[39m\u{1b}[0m\nbb   = \u{1b}[1m\u{1b}[33mstring\u{1b}[39m\u{1b}[0m\ncccc = \u{1b}[1m\u{1b}[35mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
  }

  #[test]
  fn simple_struct_custom_label_color() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: Some(Color::Green),
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: Some(Color::Yellow),
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: Some(Color::Magenta),
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    };
    //    print!("{}", s.pretty(true, None));
    assert_eq!(
    s.pretty(true,None, None).unwrap(),
 "\u{1b}[32ma\u{1b}[39m= \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n\u{1b}[33mbb\u{1b}[39m= \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\n\u{1b}[35mcccc\u{1b}[39m= \u{1b}[1m\u{1b}[97mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
  }

  #[test]
  fn option_struct() {
    struct T1 {
      a: Option<u32>,
      bb: Option<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.a.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: false,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.bb.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: Some(5),
      bb: None,
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    = 5\nbb   = null\n".to_string()
    );
  }

  #[test]
  fn formatter_option_struct() {
    struct T1 {
      a: u32,
      bb: Option<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: Some(&|x, _| Ok((format!("{} format", x.to_string()), false))),
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.bb.as_ref().map(|x| x as &dyn ToString),
                formatter: Some(&|x, _| Ok((format!("{} format", x.to_string()), false))),
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: 5,
      bb: Some("option".to_string()),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    = 5 format\nbb   = option format\n".to_string()
    );
  }

  #[test]
  fn skip_none_option_struct() {
    struct T1 {
      a: Option<u32>,
      bb: Option<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.a.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: true,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.bb.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 { a: None, bb: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "bb   = null\n".to_string()
    );
  }

  #[test]
  fn option_struct_colored() {
    struct T1 {
      a: Option<u32>,
      bb: Option<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.a.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: false,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionString {
                value: self.bb.as_ref().map(|x| x as &dyn ToString),
                formatter: None,
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: Some(5),
      bb: None,
    };
    //    print!("{}", s.pretty(true, None));
    assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a    = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nbb   = \u{1b}[35mnull\u{1b}[39m\n".to_string()
    );
  }

  #[test]
  fn nested_option_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: Option<T1>,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionPretty {
                value: self.n.as_ref().map(|x| x as &dyn PrettyPrint),
                skip_none: false,
              },
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: Some(T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      }),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
    );

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn = null\n".to_string()
    );
  }

  #[test]
  fn skip_none_nested_option_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: Option<T1>,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionPretty {
                value: self.n.as_ref().map(|x| x as &dyn PrettyPrint),
                skip_none: true,
              },
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: Some(T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      }),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
    );

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());
  }

  #[test]
  fn vec_struct() {
    struct T1 {
      a: Vec<u32>,
      bb: Vec<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::VecString(self.a.iter().map(|x| x as &dyn ToString).collect()),
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::VecString(self.bb.iter().map(|x| x as &dyn ToString).collect()),
            },
          ],
        }
      }
    }

    let s = T1 {
      a: vec![5, 3, 1, 4, 2],
      bb: vec!["a".to_string(), "string".to_string()],
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a :\n - 5\n - 3\n - 1\n - 4\n - 2\nbb :\n - a\n - string\n".to_string()
    );
  }

  #[test]
  fn vec_struct_colored() {
    struct T1 {
      a: Vec<u32>,
      bb: Vec<String>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::VecString(self.a.iter().map(|x| x as &dyn ToString).collect()),
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::VecString(self.bb.iter().map(|x| x as &dyn ToString).collect()),
            },
          ],
        }
      }
    }

    let s = T1 {
      a: vec![5, 3, 1, 4, 2],
      bb: vec!["a".to_string(), "string".to_string()],
    };
    //    print!("{}", s.pretty(true, None, None).unwrap());
    assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a :\n - \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m4\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\nbb :\n - \u{1b}[1m\u{1b}[97ma\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\n".to_string()
    );
  }

  #[test]
  fn nested_vec_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: Vec<T1>,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::VecPretty(self.n.iter().map(|x| x as &dyn PrettyPrint).collect()),
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: vec![T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      }],
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn :\n - a    = 5\n   bb   = string\n   cccc = false\n".to_string()
    );
  }

  #[test]
  fn option_vec_struct() {
    struct T1 {
      a: Option<Vec<u32>>,
      bb: Option<Vec<String>>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .a
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: false,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .bb
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: Some(vec![5, 3, 1, 4, 2]),
      bb: Some(vec!["a".to_string(), "string".to_string()]),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a :\n - 5\n - 3\n - 1\n - 4\n - 2\nbb :\n - a\n - string\n".to_string()
    );

    let s = T1 { a: None, bb: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a : null\nbb : null\n".to_string()
    );
  }

  #[test]
  fn skip_none_option_vec_struct() {
    struct T1 {
      a: Option<Vec<u32>>,
      bb: Option<Vec<String>>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .a
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: true,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .bb
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: Some(vec![5, 3, 1, 4, 2]),
      bb: Some(vec!["a".to_string(), "string".to_string()]),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a :\n - 5\n - 3\n - 1\n - 4\n - 2\nbb :\n - a\n - string\n".to_string()
    );

    let s = T1 { a: None, bb: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "bb : null\n".to_string()
    );
  }

  #[test]
  fn option_vec_struct_colored() {
    struct T1 {
      a: Option<Vec<u32>>,
      bb: Option<Vec<String>>,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .a
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: false,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecString {
                value: self
                  .bb
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn ToString).collect()),
                skip_none: false,
              },
            },
          ],
        }
      }
    }

    let s = T1 {
      a: Some(vec![5, 3, 1, 4, 2]),
      bb: Some(vec!["a".to_string(), "string".to_string()]),
    };
    //    print!("{}", s.pretty(true, None, None).unwrap());
    assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a :\n - \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m4\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\nbb :\n - \u{1b}[1m\u{1b}[97ma\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\n".to_string()
    );

    let s = T1 { a: None, bb: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a : null\nbb : null\n".to_string()
    );
  }

  #[test]
  fn nested_option_vec_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: Option<Vec<T1>>,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecPretty {
                value: self
                  .n
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn PrettyPrint).collect()),
                skip_none: false,
              },
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: Some(vec![T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      }]),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn :\n - a    = 5\n   bb   = string\n   cccc = false\n".to_string()
    );

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn : null\n".to_string()
    );

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(true, None, None).unwrap());
    assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nn :\u{1b}[35m null\n\u{1b}[39m".to_string()
    );
  }

  #[test]
  fn skip_none_nested_option_vec_struct() {
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }
    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }
    struct T2 {
      a: u32,
      n: Option<Vec<T1>>,
    }
    impl PrettyPrint for T2 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 2,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "n",
                label_color: None,
              },
              color: None,
              value: MetaValue::OptionVecPretty {
                value: self
                  .n
                  .as_ref()
                  .map(|vec| vec.iter().map(|x| x as &dyn PrettyPrint).collect()),
                skip_none: true,
              },
            },
          ],
        }
      }
    }
    let s = T2 {
      a: 5,
      n: Some(vec![T1 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      }]),
    };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a = 5\nn :\n - a    = 5\n   bb   = string\n   cccc = false\n".to_string()
    );

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(false, None, None).unwrap());
    assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());

    let s = T2 { a: 5, n: None };
    //    print!("{}", s.pretty(true, None, None).unwrap());
    assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n".to_string()
    );
  }

  #[test]
  fn coloring_none() {
    assert_eq!(
      coloring("string".to_string(), &None),
      "\u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m".to_string()
    );
  }

  #[test]
  fn coloring_some_red() {
    assert_eq!(
      coloring("string".to_string(), &Some(Color::Red)),
      "\u{1b}[1m\u{1b}[31mstring\u{1b}[39m\u{1b}[0m".to_string()
    );
  }

  #[test]
  fn simple_enum() {
    enum E1 {
      Aa,
      Bb,
    }

    impl PrettyPrint for E1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![MetaField {
            profiles: vec![],
            field_prefix: FieldPrefix::None,
            color: None,
            value: MetaValue::Variant {
              value: match self {
                E1::Aa => &"Aa",
                E1::Bb => &"Bb",
              },
              formatter: None,
            },
          }],
        }
      }
    }

    let s = E1::Aa;
    assert_eq!(s.pretty(false, None, None).unwrap(), "Aa\n".to_string());
    let s = E1::Bb;
    assert_eq!(s.pretty(false, None, None).unwrap(), "Bb\n".to_string());
  }

  #[test]
  fn enum_with_struct() {
    #[derive(Debug)]
    struct T1 {
      a: u32,
      bb: String,
      cccc: bool,
    }

    impl PrettyPrint for T1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "a",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.a,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "bb",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.bb,
                formatter: None,
              },
            },
            MetaField {
              profiles: vec![],
              field_prefix: FieldPrefix::Label {
                label: "cccc",
                label_color: None,
              },
              color: None,
              value: MetaValue::String {
                value: &self.cccc,
                formatter: None,
              },
            },
          ],
        }
      }
    }

    enum E1 {
      Aa(T1),
      Bb,
    }

    impl PrettyPrint for E1 {
      fn meta(&self) -> Meta {
        Meta {
          padding: 5,
          separator: None,
          fields: vec![MetaField {
            profiles: vec![],
            field_prefix: FieldPrefix::Label {
              label: "a",
              label_color: None,
            },
            color: None,
            value: match self {
              E1::Aa(t) => MetaValue::Pretty(t),
              E1::Bb => MetaValue::Variant {
                value: &"Bb",
                formatter: None,
              },
            },
          }],
        }
      }
    }

    let s = E1::Aa(T1 {
      a: 14,
      bb: "aaa".to_string(),
      cccc: true,
    });
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a -->\n| a    = 14\n| bb   = aaa\n| cccc = true\n".to_string()
    );
    let s = E1::Bb;
    assert_eq!(
      s.pretty(false, None, None).unwrap(),
      "a    = Bb\n".to_string()
    );
  }
}
