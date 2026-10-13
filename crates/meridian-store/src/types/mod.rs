mod game;
mod identifiers;
mod platform;
mod protocol;

pub use game::PlatformGame;
pub use identifiers::{
   PlatformGameId,
   StudioId,
};
pub use platform::Platform;
pub use protocol::{
   GameV1,
   ProtocolVersion,
   SyncEnvelopeV1,
};

#[cfg(test)] mod tests;
