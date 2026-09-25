#![feature(try_blocks)]

use std::{
   env,
   fs::File,
   io,
   path::{
      Path,
      PathBuf,
   },
   process::ExitCode,
};

use clap::{
   Parser,
   Subcommand,
};
use meridian_cli::Config;
use meridian_credential_store::FileCredentialStore;
use serde_json::from_reader;

const LINUX_CONFIG_FILE_DEFAULT_PATH: &str = "";
const WINDOWS_CONFIG_FILE_DEFAULT_PATH: &str = "";

/// Meridian Client commands.
#[derive(Debug, Parser)]
#[command(name = "meridian", version, about)]
struct Cli {
   #[command(subcommand)]
   command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
   /// Reports local runtime status.
   Status {
      #[arg(short, long)]
      config: Option<PathBuf>,
   },
   /// One-shot operation for querying and uploading data.
   Sync {
      #[arg(short, long)]
      config: Option<PathBuf>,
   },
   /// Agent mode. Self-scheduled querying and uploading functionality.
   Agent {
      #[arg(short, long)]
      config: Option<PathBuf>,
   },
}

fn main() -> ExitCode {
   let cmd = Cli::parse().command;

   // Config File Path can be specified for all commands.
   let config_file_path = match &cmd {
      &Command::Status { ref config }
      | &Command::Sync { ref config }
      | &Command::Agent { ref config } => config,
   };

   // Fallback to operating system defaults if not provided.
   let config_file_path = config_file_path.as_deref().unwrap_or_else(|| {
      match env::consts::OS {
         "linux" => Path::new(LINUX_CONFIG_FILE_DEFAULT_PATH),
         "windows" => Path::new(WINDOWS_CONFIG_FILE_DEFAULT_PATH),
         _ => Path::new(""),
      }
   });

   // Validate the config file is a exists and is a supported format.
   // Eventually, let's plan to actually parse json, but for now it is fine
   // to just check the extension.
   if !config_file_path
      .extension()
      .and_then(|ext| ext.to_str())
      .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
   {
      eprintln!("ERROR: Invalid file format. The configuration file must be a valid JSON file.");
      return ExitCode::FAILURE;
   }

   let file: Result<File, io::Error> = try { File::open(config_file_path)? };

   let Ok(file) = file else {
      eprintln!("ERROR: Invalid file format. The configuration file must be a valid JSON file.");
      return ExitCode::FAILURE;
   };

   let reader = io::BufReader::new(file);

   let config: Result<Config, serde_json::Error> = try { from_reader(reader)? };

   let Ok(config) = config else {
      let config_file_path = config_file_path.display();
      eprintln!("ERROR: Error reading file {config_file_path}.");
      return ExitCode::FAILURE;
   };

   let _credential_store = FileCredentialStore::new(PathBuf::from(config.secret_file_path));

   // TODO: match cmd { ... }

   todo!()
}
