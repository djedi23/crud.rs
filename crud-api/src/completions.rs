use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn completions_subcommand(app: Command) -> Command {
  app.subcommand(
    Command::new("completion")
      .about("Generate shell completion")
      .arg(
        Arg::new("generator")
          .long("generate")
          .action(ArgAction::Set)
          .required(true)
          .value_parser(value_parser!(Shell)),
      )
      .subcommand_precedence_over_arg(true),
  )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
  generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn generate_completions(completions: &ArgMatches, cmd: &mut Command) {
  if let Some(generator) = completions.get_one::<Shell>("generator").copied() {
    eprintln!(
      "Generating completion file for {:?}...",
      completions.subcommand()
    );
    print_completions(generator, cmd);
  }
}
