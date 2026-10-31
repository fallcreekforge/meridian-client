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
      let output = directory
         .command()
         .env_remove("HOME")
         .env_remove("XDG_CONFIG_HOME")
         .env_remove("LOCALAPPDATA")
         .arg(flag)
         .output()
         .unwrap();
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

#[cfg(target_os = "linux")]
#[test]
fn linux_status_uses_xdg_config_home_before_home() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   let xdg_config_home = directory.0.join("custom config");
   let config_directory = xdg_config_home.join("meridian");
   fs::create_dir_all(&config_directory).unwrap();
   fs::copy(
      directory.0.join("sample/config.json"),
      config_directory.join("config.json"),
   )
   .unwrap();
   let output = directory
      .command()
      .env("XDG_CONFIG_HOME", xdg_config_home)
      .env("HOME", directory.0.join("unused home"))
      .arg("status")
      .output()
      .unwrap();
   assert!(output.status.success());
   assert!(output.stderr.is_empty());
   assert_no_credentials(&output);
}

#[cfg(target_os = "linux")]
#[test]
fn linux_status_falls_back_to_home_for_unset_empty_or_relative_xdg_paths() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   let home = directory.0.join("user home");
   let config_directory = home.join(".config/meridian");
   fs::create_dir_all(&config_directory).unwrap();
   fs::copy(
      directory.0.join("sample/config.json"),
      config_directory.join("config.json"),
   )
   .unwrap();
   for xdg_config_home in [None, Some(""), Some("relative config")] {
      let mut command = directory.command();
      command.env("HOME", &home).env_remove("XDG_CONFIG_HOME");
      if let Some(value) = xdg_config_home {
         command.env("XDG_CONFIG_HOME", value);
      }
      let output = command.arg("status").output().unwrap();
      assert!(output.status.success());
      assert!(output.stderr.is_empty());
      assert_no_credentials(&output);
   }
}

#[cfg(target_os = "linux")]
#[test]
fn linux_requires_an_explicit_config_when_user_directories_are_unavailable() {
   let directory = SmokeDirectory::new();
   directory.write_samples();
   let mut command = directory.command();
   command
      .env_remove("HOME")
      .env_remove("XDG_CONFIG_HOME")
      .arg("status");
   let output = command.output().unwrap();
   assert!(!output.status.success());
   assert!(String::from_utf8_lossy(&output.stderr).contains("use --config"));

   let output = command
      .args(["--config", "sample/config.json"])
      .output()
      .unwrap();
   assert!(output.status.success());
   assert!(output.stderr.is_empty());
   assert_no_credentials(&output);
}
