use std::{
   env,
   path::PathBuf,
   process::ExitCode,
};

use clap::{
   Parser,
   Subcommand,
};
use meridian_credential_store::FileCredentialStore;
use meridian_utils::config_file::load_json_file;

mod config;
use self::config::Config;

// ########################
// #        LINUX         #
// ########################
const LINUX_CONFIG_FILE_DEFAULT_PATH: &str = "/etc/meridian/config.json";
#[expect(dead_code)]
const LINUX_SQLITE_DEFAULT_PATH: &str = "/var/lib/meridian/state.db";

// ########################
// #       WINDOWS        #
// ########################
const WINDOWS_CONFIG_FILE_DEFAULT_PATH: &str = "Meridian/config.json";
#[expect(dead_code)]
const WINDOWS_SQLITE_DEFAULT_PATH: &str = "~/AppData/Local/Meridian/config.json";

#[expect(clippy::doc_paragraphs_missing_punctuation)]
/// Meridian Client commands.
///
/// # Usage
///
/// `meridian <COMMAND> [OPTIONS]`
///
/// Supported commands are `status`, `sync`, and `agent`. Each command
/// accepts `-c, --config <CONFIG>` to specify a configuration file.
#[derive(Debug, Parser)]
#[command(name = "meridian", version, about)]
struct Cli {
   #[command(subcommand)]
   command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
   /// Checks local configuration and credential loading.
   Status {
      #[arg(short = 'c', long)]
      config: Option<PathBuf>,
   },
   /// One-shot operation for querying and uploading data.
   Sync {
      #[arg(short = 'c', long)]
      config: Option<PathBuf>,
   },
   /// Agent mode. Self-scheduled querying and uploading functionality.
   Agent {
      #[arg(short = 'c', long)]
      config: Option<PathBuf>,
   },
}

fn main() -> ExitCode {
   // Clap handles --version and -V before configuration is loaded.
   let Some(cmd) = Cli::parse().command else {
      return ExitCode::SUCCESS;
   };

   let config_file_path = match &cmd {
      &Command::Status { ref config }
      | &Command::Sync { ref config }
      | &Command::Agent { ref config } => config,
   };

   // Fallback to operating system defaults if not provided.
   let default_config_file_path = match env::consts::OS {
      "windows" => {
         env::var_os("LOCALAPPDATA")
            .map(|directory| PathBuf::from(directory).join(WINDOWS_CONFIG_FILE_DEFAULT_PATH))
      },
      "linux" => Some(PathBuf::from(LINUX_CONFIG_FILE_DEFAULT_PATH)),
      _ => None,
   };
   let Some(config_file_path) = config_file_path
      .as_deref()
      .or(default_config_file_path.as_deref())
   else {
      eprintln!("ERROR: No default configuration location is available; use --config.");
      return ExitCode::FAILURE;
   };

   let config = match load_json_file::<Config>(config_file_path) {
      Ok(config) => config,
      Err(error) => {
         eprintln!("ERROR: Could not load configuration: {error}.");
         return ExitCode::FAILURE;
      },
   };

   if let Err(error) = FileCredentialStore::load_from_file(config.credentials_file) {
      eprintln!("ERROR: Could not load credentials: {error}.");
      return ExitCode::FAILURE;
   }

   match cmd {
      Command::Status { .. } => {
         println!("Configuration and credentials loaded successfully.");
         ExitCode::SUCCESS
      },
      Command::Sync { .. } | Command::Agent { .. } => {
         eprintln!("ERROR: Sync and agent execution are not implemented yet.");
         ExitCode::FAILURE
      },
   }
}
