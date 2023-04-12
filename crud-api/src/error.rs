use clap::error::{ContextKind, ContextValue};
use crud_auth::CrudAuth;
use hyper::StatusCode;
use miette::Diagnostic;
use owo_colors::OwoColorize;
use regex::Regex;
use std::env;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ApiError {
  #[error("HTTP Status: {status:}")]
  #[diagnostic(code(http::bad_status), help("try another argument?"))]
  HTTPStatusError { status: StatusCode },
  #[error("HTTP Status: {status:}")]
  #[diagnostic(code(http::unauthorized), help("{help:}"))]
  HTTPUnauthorizedError { status: StatusCode, help: String },

  #[error("Error reading a JSON document")]
  #[diagnostic(code("deserialization error"), help("Check your JSON document"))]
  JsonSerdeDeserError {
    #[from]
    source: serde_json::Error,
  },
}

impl ApiError {
  pub fn from_http_status(
    status: StatusCode,
    auth: Option<&(dyn CrudAuth + Send + Sync)>,
  ) -> ApiError {
    match status {
      StatusCode::UNAUTHORIZED => ApiError::HTTPUnauthorizedError {
        status,
        help: if let Some(auth) = auth {
          auth.error_help_message()
        } else {
          "Check your authentification".to_string()
        },
      },
      _ => ApiError::HTTPStatusError { status },
    }
  }
}

#[derive(Debug, Error, Diagnostic)]
pub enum ClapError {
  #[error("Error parsing arguments")]
  #[diagnostic(code(cli::error))]
  Error { source: clap::Error },

  #[error(
    "'{invalid_value:}' isn't a valid value for '{invalid_arg}'\n\t[possible values: {valid_value:}]"
  )]
  #[diagnostic(code(cli::invalid_value), help("For more information try '--help'"))]
  ClapInvalidValue {
    err_string: String,
    invalid_value: String,
    invalid_arg: String,
    valid_value: String,
    suggestion: String,
    #[source_code]
    src: String,
    #[label = "Invalid value"]
    err_span: (usize, usize),
    #[label = "Did you mean \"{suggestion:}\"?"]
    suggestion_span: Option<(usize, usize)>,
  },

  #[error("The argument '{invalid_arg:}' requires a value but none was supplied")]
  #[diagnostic(code(cli::empty_value), help("For more information try '--help'"))]
  ClapEmptyValue {
    err_string: String,
    invalid_arg: String,
    #[source_code]
    src: String,

    #[label = "Requires a value"]
    err_span: (usize, usize),
  },

  #[error("Found argument '{invalid_arg:}' which wasn't expected, or isn't valid in this context\n\n\tIf you tried to supply `{invalid_arg:}` as a value rather than a flag, use `-- {invalid_arg:}`\n\n{usage:}")]
  #[diagnostic(code(cli::unknown_argument), help("For more information try '--help'"))]
  ClapUnknownArgument {
    err_string: String,
    invalid_arg: String,
    usage: String,
    suggestion: String,
    #[source_code]
    src: String,

    #[label = "Unknow argument"]
    err_span: (usize, usize),
    #[label = "Did you mean \"{suggestion:}\"?"]
    suggestion_span: Option<(usize, usize)>,
  },
  #[error("The subcommand '{invalid_sub_command:}' wasn't recognized\n\n{usage:}")]
  #[diagnostic(
    code(cli::invalid_sub_command),
    help("If you believe you received this message in error, try re-running with '{suggestion_command:}'\n\nFor more information try '--help'")
  )]
  ClapInvalidSubcommand {
    err_string: String,
    invalid_sub_command: String,
    usage: String,
    suggestion: String,
    suggestion_command: String,
    #[source_code]
    src: String,

    //     #[label = "Unknow argument"]
    //     err_span: (usize, usize),
    #[label = "Did you mean {suggestion:}?"]
    suggestion_span: Option<(usize, usize)>,
  },

  #[error("Invalid value \"{invalid_value:}\" for '{invalid_arg:}': {source_:}")]
  #[diagnostic(code(cli::value_validation), help("For more information try '--help'"))]
  ClapValueValidation {
    err_string: String,
    invalid_value: String,
    invalid_arg: String,
    source_: String,
    #[source_code]
    src: String,

    #[label = "Invalid value"]
    err_span: (usize, usize),
  },

  #[error("The argument '{invalid_arg:}' cannot be used with '{prior_arg:}'\n\n{usage:}")]
  #[diagnostic(
    code(cli::argument_conflict),
    help("For more information try '--help'")
  )]
  ClapArgumentConflict {
    err_string: String,
    invalid_arg: String,
    prior_arg: String,
    usage: String,
    #[source_code]
    src: String,

    #[label = "This argument conflicts"]
    arg_span: (usize, usize),
    //     #[label = "with this argument"]
    //     prior_span: Vec<(usize, usize)>,
  },
}

impl From<clap::Error> for ClapError {
  fn from(e: clap::Error) -> Self {
    match e.kind() {
      clap::error::ErrorKind::InvalidValue => {
        let src = env::args().collect::<Vec<String>>().join(" ");
        let invalid_value = from_context(&e, ContextKind::InvalidValue);

        if invalid_value.is_empty() {
          //	              let src = env::args().collect::<Vec<String>>().join(" ");
          let invalid_arg = from_context(&e, ContextKind::InvalidArg);
          let ia = invalid_arg.split(' ').next().unwrap_or_default();
          let re = Regex::new(ia).unwrap();
          let caps = re.find(&src);

          ClapError::ClapEmptyValue {
            err_string: e.to_string(),
            invalid_arg: invalid_arg.yellow().to_string(),
            src: src.clone(),
            err_span: if let Some(caps) = caps {
              (caps.start(), caps.end() - caps.start())
            } else {
              (src.len() - 1, 0)
            },
          }
        } else {
          let re = Regex::new(&invalid_value).unwrap();
          let caps = re.find(&src).unwrap();
          ClapError::ClapInvalidValue {
            err_string: e.to_string(),
            invalid_value: invalid_value.yellow().to_string(),
            invalid_arg: from_context(&e, ContextKind::InvalidArg)
              .yellow()
              .to_string(),
            valid_value: from_context(&e, ContextKind::ValidValue),
            suggestion: from_context(&e, ContextKind::SuggestedValue),
            src: src.clone(),
            err_span: (caps.start(), caps.end() - caps.start()),
            suggestion_span: if from_context(&e, ContextKind::SuggestedValue).is_empty() {
              None
            } else {
              Some((caps.start(), caps.end() - caps.start()))
            },
          }
        }
      }
      clap::error::ErrorKind::UnknownArgument => {
        let src = env::args().collect::<Vec<String>>().join(" ");
        let invalid_arg = from_context(&e, ContextKind::InvalidArg);
        let re = Regex::new(&invalid_arg).unwrap();
        let caps = re.find(&src).unwrap();
        ClapError::ClapUnknownArgument {
          err_string: e.to_string(),
          invalid_arg: invalid_arg.yellow().to_string(),
          usage: from_context(&e, ContextKind::Usage),
          src: src.clone(),
          err_span: (caps.start(), caps.end() - caps.start()),
          suggestion: from_context(&e, ContextKind::SuggestedArg),
          suggestion_span: if from_context(&e, ContextKind::SuggestedArg).is_empty() {
            None
          } else {
            Some((caps.start(), caps.end() - caps.start()))
          },
        }
      }
      clap::error::ErrorKind::InvalidSubcommand => {
        let src = env::args().collect::<Vec<String>>().join(" ");
        let invalid_sub_command = from_context(&e, ContextKind::InvalidSubcommand);
        let re = Regex::new(&invalid_sub_command).unwrap();
        let caps = re.find(&src).unwrap();
        ClapError::ClapInvalidSubcommand {
          err_string: e.to_string(),
          invalid_sub_command: invalid_sub_command.yellow().to_string(),
          usage: from_context(&e, ContextKind::Usage),
          src: src.clone(),
          suggestion: from_context(&e, ContextKind::SuggestedSubcommand),
          suggestion_command: from_context(&e, ContextKind::SuggestedCommand),
          suggestion_span: if from_context(&e, ContextKind::SuggestedSubcommand).is_empty() {
            None
          } else {
            Some((caps.start(), caps.end() - caps.start()))
          },
        }
      }
      clap::error::ErrorKind::NoEquals => todo!(),
      clap::error::ErrorKind::ValueValidation => {
        let src = env::args().collect::<Vec<String>>().join(" ");
        let invalid_value = from_context(&e, ContextKind::InvalidValue);
        let re = Regex::new(&invalid_value).unwrap();
        let caps = re.find(&src).unwrap();
        ClapError::ClapValueValidation {
          err_string: e.to_string(),
          invalid_value: invalid_value.yellow().to_string(),
          invalid_arg: from_context(&e, ContextKind::InvalidArg)
            .yellow()
            .to_string(),
          source_: String::new(), //e.info[2].clone(),
          src: src.clone(),
          err_span: (caps.start(), caps.end() - caps.start()),
        }
      }
      clap::error::ErrorKind::TooManyValues => ClapError::Error { source: e },
      clap::error::ErrorKind::TooFewValues => ClapError::Error { source: e },
      //      clap::error::ErrorKind::TooManyOccurrences => ClapError::Error { source: e },
      clap::error::ErrorKind::WrongNumberOfValues => ClapError::Error { source: e },
      clap::error::ErrorKind::ArgumentConflict => {
        let src = env::args().collect::<Vec<String>>().join(" ");
        let invalid_arg = from_context(&e, ContextKind::InvalidArg);
        let re = Regex::new(invalid_arg.split(' ').next().unwrap_or_default()).unwrap();
        let caps = re.find(&src).unwrap();

        let priors = vec_from_context(&e, ContextKind::PriorArg);
        let _prior_spans: Vec<(usize, usize)> = priors
          .iter()
          .filter_map(|prior| {
            let re = Regex::new(prior).unwrap();
            re.find(&src)
              .map(|prior_caps| (prior_caps.start(), prior_caps.end() - prior_caps.start()))
          })
          .collect();

        ClapError::ClapArgumentConflict {
          err_string: e.to_string(),
          invalid_arg: invalid_arg.yellow().to_string(),
          prior_arg: from_context(&e, ContextKind::PriorArg).yellow().to_string(),
          usage: from_context(&e, ContextKind::Usage),
          src: src.clone(),
          arg_span: (caps.start(), caps.end() - caps.start()),
          //          prior_span: prior_spans,
        }
      }
      clap::error::ErrorKind::MissingRequiredArgument => ClapError::Error { source: e }, // FIXME
      clap::error::ErrorKind::MissingSubcommand => ClapError::Error { source: e },       // FIXME
      //      clap::error::ErrorKind::UnexpectedMultipleUsage => ClapError::Error { source: e },
      clap::error::ErrorKind::InvalidUtf8 => ClapError::Error { source: e },
      clap::error::ErrorKind::DisplayHelp => e.exit(),
      clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => e.exit(),
      clap::error::ErrorKind::DisplayVersion => e.exit(),
      //      clap::error::ErrorKind::ArgumentNotFound => ClapError::Error { source: e }, // FIXME ?
      clap::error::ErrorKind::Io => ClapError::Error { source: e },
      clap::error::ErrorKind::Format => ClapError::Error { source: e },
      _ => ClapError::Error { source: e },
    }
  }
}

fn from_context(err: &clap::Error, kind: ContextKind) -> String {
  err
    .context()
    .find_map(|(k, c)| {
      if k == kind {
        return match c {
          ContextValue::None => Some("".to_string()),
          ContextValue::Bool(b) => Some(b.to_string()),
          ContextValue::String(s) => Some(s.to_owned()),
          ContextValue::Strings(vs) => Some(vs.join(", ")),
          ContextValue::Number(n) => Some(n.to_string()),
          ContextValue::StyledStr(s) => Some(s.ansi().to_string()),
          _ => todo!(),
        };
      }
      None
    })
    .unwrap_or_default()
}
fn vec_from_context(err: &clap::Error, kind: ContextKind) -> Vec<String> {
  match err.context().find(|&(k, _)| k == kind) {
    Some((_, c)) => match c {
      ContextValue::None => vec!["".to_string()],
      ContextValue::Bool(b) => vec![b.to_string()],
      ContextValue::String(s) => vec![s.to_owned()],
      ContextValue::Strings(vs) => vec![vs.join(", ")],
      ContextValue::Number(n) => vec![n.to_string()],
      _ => todo!(),
    },
    None => vec![],
  }
}
