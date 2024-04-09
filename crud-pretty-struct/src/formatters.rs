use miette::Result;
use owo_colors::OwoColorize;

pub fn identity_formatter(x: &dyn ToString, _: bool) -> Result<(String, bool)> {
  Ok((x.to_string(), false))
}

pub fn bool_check_formatter(b: &dyn ToString, colored: bool) -> Result<(String, bool)> {
  Ok((
    match (b.to_string().as_str(), colored) {
      ("true", true) => "✔".green().to_string(),
      ("true", false) => "✔".to_string(),
      (_, true) => "✘".red().to_string(),
      (_, false) => "✘".to_string(),
    },
    colored,
  ))
}

// pub fn suffix_formatter(suffix: String) -> Box<Formatter> {
//   Box::new(move |x: &dyn ToString, _: bool| -> Result<(String, bool)> {
//     Ok((format!("{}{suffix}", x.to_string()), false))
//   })
// }

#[cfg(feature = "chrono")]
pub fn timestamp_formatter(value: &dyn ToString, _: bool) -> Result<(String, bool)> {
  use chrono::{FixedOffset, NaiveDateTime};
  use miette::{miette, Context, IntoDiagnostic};
  let d = NaiveDateTime::parse_from_str(value.to_string().as_str(), "%s")
    .into_diagnostic()
    .with_context(|| format!("Can't parse timestamps: {}", value.to_string()))?
    .and_local_timezone(FixedOffset::east_opt(0).ok_or(miette!("Can't create timezone"))?)
    .unwrap();
  Ok((d.to_string(), false))
}

#[cfg(feature = "humantime")]
pub fn duration_formatter(value: &dyn ToString, _: bool) -> Result<(String, bool)> {
  use humantime::format_duration;
  use miette::{Context, IntoDiagnostic};
  use std::time::Duration;

  let d = Duration::from_secs_f64(
    value
      .to_string()
      .parse::<f64>()
      .into_diagnostic()
      .with_context(|| format!("Can't parse duration: {}", value.to_string()))?
      .abs(),
  );
  Ok((format_duration(d).to_string(), false))
}

#[cfg(feature = "markdown")]
pub fn markdown_formatter(value: &dyn ToString, _: bool) -> Result<(String, bool)> {
  Ok((termimad::term_text(&value.to_string()).to_string(), true))
}

#[cfg(feature = "bytesize")]
pub fn byte_formatter(value: &dyn ToString, _: bool) -> Result<(String, bool)> {
  use bytesize::ByteSize;
  use miette::{Context, IntoDiagnostic};

  let d = ByteSize(
    value
      .to_string()
      .parse::<u64>()
      .into_diagnostic()
      .with_context(|| format!("Can't parse bytes: {}", value.to_string()))?,
  );
  Ok((d.to_string(), false))
}

#[cfg(test)]
mod tests {
  #[test]
  #[cfg(feature = "humantime")]
  fn duration_formatter_u64() {
    use crate::formatters::duration_formatter;
    assert_eq!(
      duration_formatter(&"14", false).unwrap(),
      ("14s".to_string(), false)
    );
  }

  #[test]
  #[cfg(feature = "humantime")]
  fn duration_formatter_f64() {
    use crate::formatters::duration_formatter;
    assert_eq!(
      duration_formatter(&"784.15", false).unwrap(),
      ("13m 4s 150ms".to_string(), false)
    );
  }

  #[test]
  #[cfg(feature = "humantime")]
  fn duration_formatter_f64_negative() {
    use crate::formatters::duration_formatter;
    assert_eq!(
      duration_formatter(&"-784.15", false).unwrap(),
      ("13m 4s 150ms".to_string(), false)
    );
  }

  // #[test]
  // #[cfg(feature = "humantime")]
  // fn duration_formatter_err() {
  //   use crate::formatters::duration_formatter;
  //   assert_eq!(duration_formatter(&"x7", false), miette::Result::Err(()));
  // }
}
