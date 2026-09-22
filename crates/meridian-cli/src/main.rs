use clap::{
   Parser,
   Subcommand,
};

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
   Status,
   /// Reports synchronization availability.
   Sync,
}

fn main() {
   match Cli::parse().command {
      Command::Status => {
         println!("No platform connections are configured.");
      },
      Command::Sync => {
         println!("Synchronization is not configured.");
      },
   }
}
