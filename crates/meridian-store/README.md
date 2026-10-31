# meridian-store

Provides Meridian Client's local SQLite persistence with SQLx. This crate owns database access,
schema migrations, transactions, persisted records, query projections, and the transactional
outbox consumed by local orchestration. In the current workspace, it also declares the
platform-neutral identifiers, supported-platform values, and versioned envelope types shared by
persistence, platform adapters, and local orchestration. These types live in
`meridian-store::types` and are re-exported from the crate root as a temporary dependency-layer
choice. They are planned to move into an independently versioned public `meridian-types` package
shared with Meridian Cloud; storage-specific types will remain here.

Platform adapters receive ingestion handles through the `Ingest` contract and coordinate writes
for their collected data. SQLite operations and atomic source-data, checkpoint, and outbox
updates belong in this crate. The current ingestion contract and dispatch implementations are
scaffolding; their operations and transaction behavior are not implemented yet.

Atlas manages schema changes during development. Released clients apply bundled, ordered migrations
in a Flyway-style sequence. Post-release migrations are forward-only and non-destructive.
