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

Edit [`schema.sql`](schema.sql) as the declarative development schema. Versioned SQL migrations
and their checksum file belong in [`migrations/`](migrations/).

From the repository root inside the development shell, use `.local/meridian.db` as the
development database. Git ignores `.local/`, including SQLite journal and WAL files.

```nu
just db-init
just db-inspect
just db-diff
just db-apply
```

Each command accepts an optional database file path, such as `just db-diff /tmp/example.db`.
Relative paths are resolved from the repository root. Custom locations are outside the `.local/`
ignore rule. The development database location is separate from the agent's configured runtime path.

`db-init` creates the parent directory and initializes the database from `schema.sql` in a
transaction; it refuses to overwrite an existing database. `db-diff` prints the SQL needed to match
`schema.sql`. `db-apply` previews changes and asks for confirmation before applying them. Both use
an in-memory scratch database to evaluate the schema.
These helpers operate on a development database and do not generate versioned migration files.
They do not maintain SQLx migration history. Use these databases for schema experimentation;
use a separate, existing empty SQLite file to exercise runtime migration startup.

`Outbox::open` applies the embedded SQL migrations before opening its connection pool. The
migrator creates `_sqlx_migrations` automatically and records applied versions and SQL checksums;
ordinary SQLx connections do not create that table. Keep it out of `schema.sql`. The adjacent
checksum file used by development tooling is ignored by SQLx.

Startup rejects a populated database without SQLx migration history rather than inferring which
migrations have run. It also rejects missing or modified applied migrations. Pending migrations
and their history records commit in one transaction. A dedicated migration connection disables
foreign-key enforcement for table rebuilds, checks referential integrity before committing, and
closes before the runtime pool enables enforcement. Migration errors roll back the pending batch;
scripts that opt out of transactions are rejected. Back up runtime databases before upgrades:
transaction rollback does not provide downgrade migrations or undo a successful data change.

TODO: Define and test recovery when an application rollback follows a successful database
migration, including schema compatibility checks and restoration from a compatible backup.
Automatic reversal of committed migrations is unsupported.

After editing the schema, generate the PR's migration:

```nu
just db-migration create_games
just db-migrations-check
```

Generation fetches `origin/main` and requires it to be an ancestor of the branch. Update the
branch with main before generating a migration. The generator reconstructs main's migration
history in an isolated SQLite database, verifies it against main's schema, and generates one SQL
migration to the working tree's schema. It applies that migration to the isolated database and
verifies the result before writing the migration and checksum file. It never uses `.local/meridian.db`.
An absent schema and migration history on main define an empty bootstrap baseline.

A PR may add at most one SQL migration. Existing merged migrations cannot be edited, deleted, or
reordered. The checksum file is excluded from the SQL file count. Code-only changes require no
migration; a schema change must be reproducible by replaying the PR migrations from its base.
Preserve hand-written SQL before removing an unmerged migration and regenerating it.
Views and triggers require explicit migration SQL; their stored definitions must match
`schema.sql` exactly. The replay check compares these definitions directly in SQLite.

The PR workflow checks this policy and schema replay against the PR's base commit. Require the
`Migration policy and replay` status check in repository branch protection to block merges on
failure. Run `just db-migrations-check <base-revision>` to check a specific base locally.

Repository-wide end-to-end tests and synthetic fixture scripts belong in the top-level
[`tests/`](../../tests/README.md) directory.

Database tests in [`src/database_tests/`](src/database_tests/mod.rs) exercise the `Outbox`
connection pool with isolated temporary files. They run bundled migrations and add synthetic
tables from [`tests/fixtures/database.sql`](tests/fixtures/database.sql), then verify literal file
paths, committed data after reopening, foreign-key enforcement, rollback, and open failures.
Separate migration fixtures verify history creation, replay, checksum validation, unversioned
database rejection, atomic failure, and preservation of referencing rows during table rebuilds.
These tests cover connection, migration, and transaction behavior; domain ingestion and dispatch guarantees
need tests against their database operations.

Express reusable database operations with parameterized SQL and Rust transaction methods.
SQLite does not provide a `CREATE PROCEDURE` statement.
