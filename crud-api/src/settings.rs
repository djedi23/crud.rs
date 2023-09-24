extern crate directories;
use clap::ArgMatches;
use config::{Config, Environment, File};
use directories::ProjectDirs;
use log::{debug, trace};
use miette::{bail, IntoDiagnostic, Result};
use std::path::Path;

pub fn settings(
  qualifier: &str,
  organisation: &str,
  application: &str,
  env_prefix: &str,
) -> Result<Config> {
  let mut settings_builder = Config::builder();
  if let Some(proj_dirs) = ProjectDirs::from(qualifier, organisation, application) {
    let path = Path::new(proj_dirs.config_dir()).join("settings.toml");
    let path = path.to_str().unwrap();
    settings_builder = settings_builder.add_source(File::with_name(path).required(false));
    settings_builder = settings_builder
      .set_default("configuration_path", path)
      .into_diagnostic()?;

    debug!("Try to load config file: {}", &path);
  }
  settings_builder = settings_builder.add_source(Environment::with_prefix(env_prefix));
  settings_builder.build().into_diagnostic()
}

pub fn get_settings(settings: &Config, matches: &ArgMatches, arg: &str) -> Result<String> {
  if let Some(value) = matches.get_one::<String>(arg) {
    Ok(value.clone())
  } else if let Some(profile) = matches.get_one::<String>("profile") {
    trace!("profile: {profile}");
    if let Ok(value) = settings.get_string(&format!("profile.{profile}.{arg}")) {
      Ok(value)
    } else if let Ok(value) = settings.get_string(arg) {
      trace!("profile {profile} not found. Fallback to default profile.");
      Ok(value)
    } else {
      bail!("Setting not found")
    }
  } else if let Ok(value) = settings.get_string(arg) {
    Ok(value)
  } else {
    bail!("Setting not found")
  }
}
