#![expect(
   clippy::tests_outside_test_module,
   reason = "This is an integration test crate."
)]

use std::{
   env,
   fs,
   path::PathBuf,
   process::{
      self,
      Command,
      Output,
   },
   sync::atomic::{
      AtomicU64,
      Ordering,
   },
};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct SmokeDirectory(PathBuf);

impl SmokeDirectory {
   #[expect(
      clippy::create_dir,
      reason = "Fail if the isolated test directory already exists."
   )]
   fn new() -> Self {
      let path = env::temp_dir().join(format!(
         "meridian smoke {} {}",
         process::id(),
         NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
      ));
      fs::create_dir(&path).expect("create isolated smoke-test directory");
      Self(path)
   }

   fn write_samples(&self) {
      let sample = self.0.join("sample");
      fs::create_dir_all(&sample).unwrap();
      fs::write(
         sample.join("config.json"),
         include_str!("../../../sample/config.json"),
      )
      .unwrap();
      fs::write(
         sample.join("secrets.json"),
         include_str!("../../../sample/secrets.json"),
      )
      .unwrap();
   }

   fn command(&self) -> Command {
      let mut command = Command::new(env!("CARGO_BIN_EXE_meridian"));
      command.current_dir(&self.0);
      command
   }
}

impl Drop for SmokeDirectory {
   fn drop(&mut self) {
      fs::remove_dir_all(&self.0).expect("remove isolated smoke-test directory");
   }
}

fn assert_no_credentials(output: &Output) {
   assert!(!String::from_utf8_lossy(&output.stdout).contains("synthetic-smoke"));
   assert!(!String::from_utf8_lossy(&output.stderr).contains("synthetic-smoke"));
}

#[test]
fn version_flags_work_without_configuration_or_credentials() {
   let directory = SmokeDirectory::new();
   for flag in ["--version", "-V"] {
      let output = directory.command().arg(flag).output().unwrap();
      assert!(output.status.success());
      assert_eq!(
         String::from_utf8(output.stdout).unwrap().trim(),
         format!("meridian {}", env!("CARGO_PKG_VERSION"))
      );
      assert!(output.stderr.is_empty());
   }
}

#[test]
fn status_loads_sample_configuration_and_credentials_offline() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   let output = directory
      .command()
      .args(["status", "--config", "sample/config.json"])
      .output()
      .unwrap();
   assert!(output.status.success());
   assert_eq!(
      String::from_utf8_lossy(&output.stdout).trim(),
      "Configuration and credentials loaded successfully."
   );
   assert!(output.stderr.is_empty());
   assert_no_credentials(&output);
}

#[test]
fn status_fails_for_missing_configuration_or_credentials() {
   let directory = SmokeDirectory::new();
   let output = directory
      .command()
      .args(["status", "--config", "sample/config.json"])
      .output()
      .unwrap();
   assert!(!output.status.success());
   assert!(String::from_utf8_lossy(&output.stderr).contains("Could not load configuration"));

   directory.write_samples();
   fs::remove_file(directory.0.join("sample/secrets.json")).unwrap();
   let output = directory
      .command()
      .args(["status", "--config", "sample/config.json"])
      .output()
      .unwrap();
   assert!(!output.status.success());
   assert!(String::from_utf8_lossy(&output.stderr).contains("Could not load credentials"));
   assert_no_credentials(&output);
}

#[test]
fn malformed_credentials_are_not_echoed_in_diagnostics() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   fs::write(
      directory.0.join("sample/secrets.json"),
      r#"{"meridianCloudKey": "synthetic-smoke-invalid-secret"}"#,
   )
   .unwrap();
   let output = directory
      .command()
      .args(["status", "--config", "sample/config.json"])
      .output()
      .unwrap();
   assert!(!output.status.success());
   assert!(String::from_utf8_lossy(&output.stderr).contains("Could not load credentials"));
   assert_no_credentials(&output);
}

#[test]
fn unfinished_runtime_commands_fail_without_panicking() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   for command in ["sync", "agent"] {
      let output = directory
         .command()
         .args([command, "--config", "sample/config.json"])
         .output()
         .unwrap();
      assert!(!output.status.success());
      assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented yet"));
      assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
      assert_no_credentials(&output);
   }
}

#[cfg(target_os = "windows")]
#[test]
fn windows_status_uses_local_app_data_for_the_default_configuration() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   let local_app_data = directory.0.join("local app data");
   let config_directory = local_app_data.join("Meridian");
   fs::create_dir_all(&config_directory).unwrap();
   fs::copy(
      directory.0.join("sample/config.json"),
      config_directory.join("config.json"),
   )
   .unwrap();
   let output = directory
      .command()
      .env("LOCALAPPDATA", local_app_data)
      .arg("status")
      .output()
      .unwrap();
   assert!(output.status.success());
   assert!(output.stderr.is_empty());
   assert_no_credentials(&output);
}
