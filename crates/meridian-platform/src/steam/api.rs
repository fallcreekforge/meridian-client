use thiserror::Error;

#[derive(Debug, Error)]
pub enum SteamError {
   #[error("Unrecognized game listed in configuration file")]
   UnrecognizedGame,
}

/// Injection boundary for Steam data access; no request capabilities are
/// defined.
pub trait SteamClient: Send + Sync {}
