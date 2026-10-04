# meridian-store

Provides Meridian Client's local SQLite persistence with SQLx. This crate owns database access,
schema migrations, transactions, persisted records, query projections, and the transactional
outbox consumed by local orchestration.

Atlas manages schema changes during development. Released clients apply bundled, ordered migrations
in a Flyway-style sequence. Post-release migrations are forward-only and non-destructive.
