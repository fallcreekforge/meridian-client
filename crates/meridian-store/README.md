# meridian-store

Provides Meridian Client's local SQLite persistence with SQLx. This crate owns database access,
schema migrations, transactions, persisted records, query projections, and the transactional
outbox consumed by local orchestration. It also declares the platform-neutral identifiers,
supported-platform values, and versioned envelope types shared by
persistence, platform adapters, and local orchestration. These types live in
`meridian-store::types` and are re-exported from the crate root as a temporary dependency-layer
choice. They are planned to move into an independently versioned public `meridian-types` package
shared with Meridian Cloud; storage-specific types will remain here.

Platform adapters receive ingestion handles through the `Ingest` contract and coordinate writes
for their collected data. SQLite operations and atomic source-data, checkpoint, and outbox
updates belong in this crate. The ingestion and dispatch traits are marker contracts with no
database operations.

Develop the schema declaratively. Released clients apply bundled migrations in version order.
Post-release migrations are forward-only and non-destructive.

Edit [`schema.sql`](schema.sql) as the declarative development schema. Before the first release,
generate a baseline migration from the agreed schema in [`migrations/`](migrations/).
Repository-wide end-to-end tests and synthetic fixture scripts belong in the top-level
[`tests/`](../../tests/README.md) directory.

Database tests in [`src/database_tests/`](src/database_tests/mod.rs) exercise the `Outbox`
connection pool with isolated temporary files. They apply `schema.sql` and the synthetic
tables in [`tests/fixtures/database.sql`](tests/fixtures/database.sql), then verify literal file
paths, committed data after reopening, foreign-key enforcement, rollback, and open failures.
These tests cover connection and transaction behavior; domain ingestion and dispatch guarantees
need tests against their database operations.

Express reusable database operations with parameterized SQL and Rust transaction methods.
SQLite does not provide a `CREATE PROCEDURE` statement.
