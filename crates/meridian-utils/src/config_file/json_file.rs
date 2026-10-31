use std::{
   fs::File,
   io::{
      self,
      BufReader,
      Error,
   },
   path::Path,
};

use serde::de::DeserializeOwned;
use thiserror::Error;

/// File-loading failures that never retain deserializer messages or input
/// values.
#[derive(Debug, Error)]
pub enum JsonFileError {
   #[error("file must have a .json extension")]
   IncorrectFileType,
   #[error("could not read JSON file: {0}")]
   Io(#[from] Error),
   #[error("invalid JSON or configuration schema at line {line}, column {column}")]
   InvalidJson { line: usize, column: usize },
}

/// Loads a typed JSON document without logging paths or file contents.
pub fn load_json_file<T>(path: impl AsRef<Path>) -> Result<T, JsonFileError>
where
   T: DeserializeOwned,
{
   let path = path.as_ref();
   if !path
      .extension()
      .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
   {
      return Err(JsonFileError::IncorrectFileType);
   }

   let reader = BufReader::new(File::open(path)?);
   serde_json::from_reader(reader).map_err(|error| {
      if error.is_io() {
         JsonFileError::Io(Error::from(
            error.io_error_kind().unwrap_or(io::ErrorKind::Other),
         ))
      } else {
         JsonFileError::InvalidJson {
            line:   error.line(),
            column: error.column(),
         }
      }
   })
}
