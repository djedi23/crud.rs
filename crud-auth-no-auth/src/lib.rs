//! ## Crud no auth
//!
//! Authentification trait implementation for [crud-api](../crud-api): no authentification.
//!

use clap::{ArgMatches, Command};
use config::Config;
use crud_auth::CrudAuth;

#[derive(Default, Debug)]
pub struct Auth {}

impl CrudAuth for Auth {
  fn clap_auth(&self, app: Command) -> Command {
    app
  }

  fn clap_matches(&mut self, _matches: &ArgMatches, _app: &mut Command, _settings: &Config) {}

  fn auth_header(&self) -> (String, String) {
    ("".to_string(), "".to_string())
  }

  fn error_help_message(&self) -> String {
    "".to_string()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
