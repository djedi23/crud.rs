use crate::error::ApiError;
use clap::{builder::PossibleValuesParser, Arg, ArgMatches, Command};
use miette::{Context, IntoDiagnostic, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
  fs::File,
  io::{stdin, stdout, BufReader},
  str::FromStr,
};

#[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
#[derive(Clone)]
pub enum OutputFormat {
  #[cfg(feature = "json")]
  Json,
  #[cfg(feature = "toml")]
  Toml,
  #[cfg(feature = "yaml")]
  Yaml,
  #[cfg(feature = "csv")]
  Csv,
  #[cfg(feature = "csv")]
  Tsv,
}

#[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
impl FromStr for OutputFormat {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      #[cfg(feature = "json")]
      "json" => Ok(OutputFormat::Json),
      #[cfg(feature = "toml")]
      "toml" => Ok(OutputFormat::Toml),
      #[cfg(feature = "yaml")]
      "yaml" => Ok(OutputFormat::Yaml),
      #[cfg(feature = "csv")]
      "csv" => Ok(OutputFormat::Csv),
      #[cfg(feature = "csv")]
      "tsv" => Ok(OutputFormat::Tsv),
      _ => Err(format!("Invalid variant: {s}")),
    }
  }
}

#[cfg(any(feature = "json", feature = "toml", feature = "yaml", feature = "csv"))]
pub fn clap_output_format_decl(
  command: Command,
  formats: Option<&[&'static str]>,
  long: &'static str,
  short: char,
  heading: &'static str,
) -> Command {
  command.arg(
    Arg::new("output_format")
      .long(long)
      .short(short)
      .help_heading(heading)
      .help("Output format (default: toml or table)")
      .action(clap::ArgAction::Set)
      .value_parser(PossibleValuesParser::new(if let Some(formats) = formats {
        formats //.iter().map(|x| *x).collect::<[&str]>()
      } else {
        #[allow(clippy::needless_borrow)]
        &[
          #[cfg(feature = "json")]
          "json",
          #[cfg(feature = "toml")]
          "toml",
          #[cfg(feature = "yaml")]
          "yaml",
          #[cfg(feature = "csv")]
          "csv",
          #[cfg(feature = "csv")]
          "tsv",
        ]
      })),
  )
}
#[cfg(all(
  not(feature = "json"),
  not(feature = "toml"),
  not(feature = "yaml"),
  not(feature = "csv")
))]
pub fn clap_output_format_decl(
  command: Command,
  formats: Option<&[&str]>,
  long: &str,
  short: char,
  heading: &str,
) -> Command {
  command
}

pub fn clap_match_output_format(argmatches: &ArgMatches) -> Option<OutputFormat> {
  argmatches
    .get_one::<String>("output_format")
    .map(|o| OutputFormat::from_str(o).unwrap())
}

pub fn clap_match_input_from_file<T: DeserializeOwned>(argmatches: &ArgMatches) -> Result<Option<T>> {
  if let Some(filename) = &argmatches.get_one::<String>("input_file").cloned() {
    // #[allow(clippy::explicit_auto_deref)]
    //    let filename: &str = &argmatches.get_one::<String>("input_file").cloned().unwrap();
    Ok(if filename == "-" {
      serde_json::from_reader(stdin())
        .map_err(ApiError::from)
        .context("Can't read JSON from stdin")?
    } else {
      let file = File::open(filename).into_diagnostic()?;
      serde_json::from_reader(BufReader::new(file))
        .map_err(ApiError::from)
        .with_context(|| format!("Can't read JSON from file '{filename}'"))?
    })
  } else {
    Ok(None)
  }
}

pub fn clap_match_template<T: Serialize + Default>(argmatches: &ArgMatches) -> Result<bool> {
  if let Some(true) = argmatches.get_one::<bool>("input_template") {
    serde_json::to_writer_pretty(stdout(), &T::default()).into_diagnostic()?;
    std::process::exit(0);
  } else {
    Ok(false)
  }
}
