//! Steam data access inside the customer environment.

use thiserror::Error;

pub mod adapter;
pub mod web_api;

#[expect(
   clippy::module_name_repetitions,
   reason = "Preserve the Steam-prefixed name re-exported at the crate root."
)]
#[derive(Debug, Error)]
pub enum SteamError {
   #[error("Unrecognized game listed in configuration file")]
   UnrecognizedGame,
}

/// Injected contract for Steam data access.
#[expect(
   clippy::module_name_repetitions,
   reason = "Preserve the Steam-prefixed name re-exported at the crate root."
)]
pub trait SteamClient: Send + Sync {}
