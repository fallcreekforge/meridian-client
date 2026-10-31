use std::{
   env,
   error::Error as _,
   fs,
   path::PathBuf,
   process,
   sync::atomic::{
      AtomicU64,
      Ordering,
   },
};

use serde::Deserialize;

use super::{
   JsonFileError,
   load_json_file,
};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

struct TestFile(PathBuf);

impl TestFile {
   fn new(extension: &str, contents: &str) -> Self {
      let path = env::temp_dir().join(format!(
         "meridian-utils-{}-{}.{}",
         process::id(),
         NEXT_FILE.fetch_add(1, Ordering::Relaxed),
         extension
      ));
      fs::write(&path, contents).expect("write synthetic fixture");
      Self(path)
   }
}

impl Drop for TestFile {
   fn drop(&mut self) {
      fs::remove_file(&self.0).expect("remove synthetic fixture");
   }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Settings {
   enabled: bool,
}

#[test]
fn loads_typed_json_with_case_insensitive_extension() {
   let file = TestFile::new("JSON", r#"{"enabled":true}"#);
   assert_eq!(load_json_file::<Settings>(&file.0).unwrap(), Settings {
      enabled: true,
   });
}

#[test]
fn rejects_other_extensions_before_opening() {
   assert!(matches!(
      load_json_file::<Settings>("missing.toml"),
      Err(JsonFileError::IncorrectFileType)
   ));
}

#[test]
fn reports_missing_files_as_io_errors() {
   let file = TestFile::new("json", "{}");
   let missing = file.0.join("missing.json");
   assert!(matches!(
      load_json_file::<Settings>(missing),
      Err(JsonFileError::Io(_))
   ));
}

#[test]
fn rejects_invalid_json_and_trailing_content() {
   for contents in ["{", r#"{"enabled":true} trailing"#] {
      let file = TestFile::new("json", contents);
      assert!(matches!(
         load_json_file::<Settings>(&file.0),
         Err(JsonFileError::InvalidJson { .. })
      ));
   }
}

#[test]
fn schema_errors_never_expose_input_values_or_sources() {
   for contents in [
      r#"{"enabled":"synthetic-secret-sentinel"}"#,
      r#"{"enabled":true,"synthetic-secret-sentinel":true}"#,
   ] {
      let file = TestFile::new("json", contents);
      let error = load_json_file::<Settings>(&file.0).unwrap_err();
      assert!(matches!(error, JsonFileError::InvalidJson { .. }));
      assert!(!format!("{error} {error:?}").contains("synthetic-secret-sentinel"));
      assert!(error.source().is_none());
   }
}
