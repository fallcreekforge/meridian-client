#![expect(
   clippy::tests_outside_test_module,
   reason = "This is an integration test crate."
)]

use std::{
   env,
   fs,
   path::{
      Path,
      PathBuf,
   },
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

   fn with_samples() -> Self {
      let directory = Self::new();
      let sample = directory.0.join("sample");
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
      directory
   }

   fn command(&self) -> Command {
      let mut command = Command::new(env!("CARGO_BIN_EXE_meridian"));
      command.current_dir(&self.0);
      for variable in ["HOME", "XDG_CONFIG_HOME", "XDG_STATE_HOME", "LOCALAPPDATA"] {
         command.env_remove(variable);
      }
      command
   }

   fn status(&self) -> Command {
      let mut command = self.command();
      command.args(["status", "--config", "sample/config.json"]);
      command
   }

   fn set_database_path(&self, path: Option<&Path>) {
      let config_path = self.0.join("sample/config.json");
      let mut config: serde_json::Value =
         serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
      if let Some(path) = path {
         config["databasePath"] = serde_json::to_value(path).unwrap();
      } else {
         config.as_object_mut().unwrap().remove("databasePath");
      }
      fs::write(config_path, serde_json::to_vec(&config).unwrap()).unwrap();
   }

   #[cfg(any(target_os = "linux", target_os = "windows"))]
   fn copy_config_to(&self, directory: &Path) {
      fs::create_dir_all(directory).unwrap();
      fs::copy(
         self.0.join("sample/config.json"),
         directory.join("config.json"),
      )
      .unwrap();
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

fn assert_success(output: &Output) {
   assert!(
      output.status.success(),
      "{}",
      String::from_utf8_lossy(&output.stderr)
   );
   assert!(output.stderr.is_empty());
   assert_no_credentials(output);
}

fn assert_failure(output: &Output, message: &str) {
   assert_eq!(output.status.code(), Some(1_i32), "{output:?}");
   assert!(
      String::from_utf8_lossy(&output.stderr).contains(message),
      "{output:?}"
   );
   assert!(output.stdout.is_empty());
   assert_no_credentials(output);
}

fn assert_database_path(output: &Output, expected: &Path) {
   assert_success(output);
   let stdout = String::from_utf8_lossy(&output.stdout);
   let reported = stdout
      .lines()
      .find_map(|line| line.strip_prefix("Database path: "))
      .expect("status must report the database path");
   assert_eq!(Path::new(reported), expected);
}

#[test]
fn version_flags_work_without_configuration_or_credentials() {
   let directory = SmokeDirectory::new();
   for flag in ["--version", "-V"] {
      let output = directory.command().arg(flag).output().unwrap();
      assert_success(&output);
      assert_eq!(
         String::from_utf8_lossy(&output.stdout).trim(),
         format!("meridian {}", env!("CARGO_PKG_VERSION"))
      );
   }
}

#[test]
fn status_loads_sample_configuration_and_credentials_offline() {
   let directory = SmokeDirectory::with_samples();
   let output = directory.status().output().unwrap();
   assert_database_path(&output, &Path::new("sample").join("state.db"));
   assert!(
      String::from_utf8_lossy(&output.stdout)
         .contains("Configuration and credentials loaded successfully.")
   );
}

#[test]
fn database_overrides_resolve_from_config_without_creating_files() {
   let directory = SmokeDirectory::with_samples();
   for (path, expected) in [
      (
         PathBuf::from("volume/state.db"),
         directory.0.join("sample").join("volume/state.db"),
      ),
      (
         directory.0.join("persistent/state.db"),
         directory.0.join("persistent/state.db"),
      ),
   ] {
      directory.set_database_path(Some(&path));
      let output = directory
         .command()
         .args(["status", "--config"])
         .arg(directory.0.join("sample/config.json"))
         .output()
         .unwrap();
      assert_database_path(&output, &expected);
      assert!(!expected.parent().unwrap().exists());
   }
}

#[test]
fn invalid_database_paths_require_an_override() {
   let directory = SmokeDirectory::with_samples();
   for (path, message) in [
      (None, "set databasePath"),
      (Some(Path::new("")), "databasePath must not be empty"),
   ] {
      directory.set_database_path(path);
      assert_failure(&directory.status().output().unwrap(), message);
   }
}

#[test]
fn status_rejects_missing_files_and_malformed_credentials_without_echoing_secrets() {
   let empty = SmokeDirectory::new();
   assert_failure(
      &empty.status().output().unwrap(),
      "Could not load configuration",
   );

   let directory = SmokeDirectory::with_samples();
   let credentials = directory.0.join("sample/secrets.json");
   fs::remove_file(&credentials).unwrap();
   assert_failure(
      &directory.status().output().unwrap(),
      "Could not load credentials",
   );

   fs::write(
      credentials,
      r#"{"meridianCloudKey": "synthetic-smoke-invalid-secret"}"#,
   )
   .unwrap();
   assert_failure(
      &directory.status().output().unwrap(),
      "Could not load credentials",
   );
}

#[test]
fn unfinished_runtime_commands_fail_without_panicking() {
   let directory = SmokeDirectory::with_samples();
   for command in ["sync", "agent"] {
      let output = directory
         .command()
         .args([command, "--config", "sample/config.json"])
         .output()
         .unwrap();
      assert_failure(&output, "not implemented yet");
   }
}

#[cfg(target_os = "windows")]
#[test]
fn windows_defaults_use_local_app_data() {
   let directory = SmokeDirectory::with_samples();
   directory.set_database_path(None);
   let local_app_data = directory.0.join("local app data");
   directory.copy_config_to(&local_app_data.join("Meridian"));
   let output = directory
      .command()
      .env("LOCALAPPDATA", &local_app_data)
      .arg("status")
      .output()
      .unwrap();
   assert_database_path(&output, &local_app_data.join("Meridian/state.db"));
   assert!(!local_app_data.join("Meridian/state.db").exists());
}

#[cfg(target_os = "linux")]
#[test]
fn linux_config_defaults_use_absolute_xdg_config_home_or_home() {
   let directory = SmokeDirectory::with_samples();
   let home = directory.0.join("user home");
   let custom = directory.0.join("custom config");
   let fallback = home.join(".config/meridian");
   for (xdg, config_directory) in [
      (None, fallback.clone()),
      (Some(Path::new("")), fallback.clone()),
      (Some(Path::new("relative")), fallback),
      (Some(custom.as_path()), custom.join("meridian")),
   ] {
      directory.copy_config_to(&config_directory);
      let mut command = directory.command();
      command.env("HOME", &home);
      if let Some(path) = xdg {
         command.env("XDG_CONFIG_HOME", path);
      }
      let output = command.arg("status").output().unwrap();
      assert_database_path(&output, &config_directory.join("state.db"));
      // Each case must find its own configuration, even after a fallback case
      // ran.
      fs::remove_file(config_directory.join("config.json")).unwrap();
   }
}

#[cfg(target_os = "linux")]
#[test]
fn linux_database_defaults_use_absolute_xdg_state_home_or_home() {
   let directory = SmokeDirectory::with_samples();
   directory.set_database_path(None);
   let home = directory.0.join("user home");
   let custom = directory.0.join("custom state");
   let fallback = home.join(".local/state/meridian/state.db");
   for (xdg, expected) in [
      (None, fallback.clone()),
      (Some(Path::new("")), fallback.clone()),
      (Some(Path::new("relative")), fallback),
      (Some(custom.as_path()), custom.join("meridian/state.db")),
   ] {
      let mut command = directory.status();
      command.env("HOME", &home);
      if let Some(path) = xdg {
         command.env("XDG_STATE_HOME", path);
      }
      let output = command.output().unwrap();
      assert_database_path(&output, &expected);
      assert!(!expected.exists());
   }
}

#[test]
fn explicit_config_works_without_a_default_location() {
   let directory = SmokeDirectory::with_samples();
   let output = directory.command().arg("status").output().unwrap();
   assert_failure(&output, "use --config");
   assert_success(&directory.status().output().unwrap());
}
