# meridian-platform

Defines the common contract between platform integrations and local orchestration.

Platform support is explicit, typed, and limited to capabilities required by local orchestration.
Adapters consume the foundation types owned by `meridian-store`, keep credentials inside
studio-controlled infrastructure, and isolate concrete API behavior in platform-specific
submodules.

Each adapter owns its typed platform client and an injected `meridian-store::Ingest` handle.
Local orchestration polls the platform-neutral `Producer` contract; the adapter coordinates
collection and ingestion through the store boundary. Database queries and transaction mechanics
remain in `meridian-store`. `SteamSource` establishes this ownership, but ingestion operations
and the collection-to-persistence path are not implemented yet.

`Platform`, `PlatformGame`, `PlatformGameId`, and `Ingest` are re-exported from `meridian-store`
under their original names for callers of this crate's public API. These are the same types and
trait, so callers do not need a direct store dependency just to use or implement them.
