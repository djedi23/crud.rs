//! ## Crud bearer authentification.
//!
//! Authentification trait implementation for [crud-api](../crud-api):
//!
//! ### Configuration
//!
//! | configuration | cli        | description         |
//! |---------------|------------|---------------------|
//! | auth_token    | auth-token | Authorization token |
//!
//!

use clap::{Arg, ArgAction, ArgMatches, Command};
use config::Config;
use crud_auth::CrudAuth;
use owo_colors::OwoColorize;

#[derive(Default, Debug)]
pub struct Auth {
  header: (String, String),
}

const AUTH_TOKEN_ARG: &str = "auth_token";
const AUTH_TOKEN_SETTING: &str = "auth_token";

impl CrudAuth for Auth {
  fn clap_auth(&self, app: Command) -> Command {
    app.arg(
      Arg::new(AUTH_TOKEN_ARG)
        .short('t')
        .long("auth-token")
        .action(ArgAction::Set)
        .help("Authorization token")
        .help_heading("Configuration"),
    )
  }

  fn clap_matches(&mut self, matches: &ArgMatches, _app: &mut Command, settings: &Config) {
    if let Some(token) = matches.get_one::<String>(AUTH_TOKEN_ARG).cloned() {
      self.header = ("Authorization".to_string(), "Bearer ".to_string() + &token);
    } else if let Ok(token) = settings.get_string(AUTH_TOKEN_SETTING) {
      self.header = ("Authorization".to_string(), "Bearer ".to_string() + &token);
    }
  }

  fn auth_header(&self) -> (String, String) {
    self.header.clone()
  }

  fn error_help_message(&self) -> String {
    format!("Use or check the `{}` argument.", "--auth-token".yellow())
  }
}

#[cfg(feature = "save_token")]
impl Auth {
  /// Save the token in the configuration file.
  pub fn save_token(token: &str, settings: &Config) -> miette::Result<()> {
    use miette::IntoDiagnostic;
    use std::{fs, path::Path};
    use toml_edit::{value, Document};

    let config_path = settings
      .get_string("configuration_path")
      .into_diagnostic()?;
    let config_str = fs::read_to_string(&config_path).unwrap_or_default();

    let mut doc = config_str.parse::<Document>().into_diagnostic()?;
    doc[AUTH_TOKEN_SETTING] = value(token);

    let path = Path::new(&config_path).parent().unwrap();
    fs::create_dir_all(path).into_diagnostic()?;
    fs::write(config_path, doc.to_string()).into_diagnostic()?;

    Ok(())
  }
}
