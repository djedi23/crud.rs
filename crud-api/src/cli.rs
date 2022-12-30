use crate::{completions::completions_subcommand, error::ClapError};
use clap::{crate_name, ArgMatches, Command};
use miette::Result;

pub fn init_clap() -> Command {
  let mut command = Command::new(crate_name!())
    .help_template(
      "\
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\
    ",
    )
    .subcommand_precedence_over_arg(true);
  command = completions_subcommand(command);
  command
}

pub fn get_matches(commands: &Command) -> Result<ArgMatches> {
  commands.clone().try_get_matches().map_err(|e| {
    let e: ClapError = e.into();
    e.into()
  })
}
