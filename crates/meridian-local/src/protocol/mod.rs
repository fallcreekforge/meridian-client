//! The explicit, versioned contract allowed to cross into Meridian Cloud.
//!
//! Changes to these types can expand the customer-to-cloud trust boundary and
//! require security review. Platform authentication material is deliberately
//! absent from this type system.

mod types;

pub use types::{
   GamePlatformV1,
   GameV1,
   ProtocolVersion,
   StudioId,
   SyncEnvelopeV1,
};

#[cfg(test)] mod tests;
