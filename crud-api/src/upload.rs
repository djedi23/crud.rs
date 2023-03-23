use crate::{ApiInput, ApiInputOptions};
use base64::{engine::general_purpose, write::EncoderStringWriter};
use clap::{ArgMatches, Command};
pub use crud_api_derive::*;
use miette::Result;
use serde::{ser, Deserialize, Serialize};
use std::{
  fmt::Debug,
  fs::File,
  io::{copy, stdin},
  marker::Sized,
  str::FromStr,
};

/// Upload a stream as Base64.
///
/// In your `ApiInput`s, declare a field with type `UploadBase64`.
/// This field accepts a file name or `-` for _stdin_. It will be encoded as base64 and transported in the JSON payload.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UploadBase64 {
  file: String,
}

impl ApiInput for UploadBase64 {
  fn clap(_app: Command, _options: Option<ApiInputOptions>) -> Command {
    todo!()
  }

  fn from_clap_matches(_matches: &ArgMatches) -> Result<Self>
  where
    Self: Sized,
  {
    todo!()
  }
}

impl Serialize for UploadBase64 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut buf = String::new();
    let mut enc = EncoderStringWriter::from_consumer(&mut buf, &general_purpose::STANDARD);
    if self.file == "-" {
      copy(&mut stdin(), &mut enc).map_err(ser::Error::custom)?;
    } else {
      let mut file = File::open(&self.file).map_err(ser::Error::custom)?;
      copy(&mut file, &mut enc).map_err(ser::Error::custom)?;
    }
    enc.into_inner();
    serializer.serialize_str(&buf)
  }
}

impl FromStr for UploadBase64 {
  type Err = miette::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(UploadBase64 {
      file: s.to_string(),
    })
  }
}
