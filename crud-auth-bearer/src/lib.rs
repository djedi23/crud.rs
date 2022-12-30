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
    let app = app.arg(
      Arg::new(AUTH_TOKEN_ARG)
        .short('t')
        .long("auth-token")
        .action(ArgAction::Set)
        .help("Authorization token")
        .help_heading("Configuration"),
    );
    app
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
