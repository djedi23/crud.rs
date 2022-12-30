//! ## Crud auth trait
//!
//! Authentification trait crate for [crud-api](../crud-api).
//!

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use config::Config;

/// Authification module for Crud.
/// Import an implementation of this trait to manage authification.
/// TODO: Add some examples
#[async_trait]
pub trait CrudAuth {
  /// Create the arguments for the CLI.
  /// For examples, it can create the `--login`, `--password` arguments.
  fn clap_auth(&self, app: Command) -> Command;

  /// Process the arguments created by [clap_auth] function.
  fn clap_matches(&mut self, args: &ArgMatches, app: &mut Command, settings: &Config);

  /// Create an authentification header as a pair (_name_,_valeur_)
  fn auth_header(&self) -> (String, String);

  /// The helpmessage displayed when the user is unauthorized.
  fn error_help_message(&self) -> String;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
