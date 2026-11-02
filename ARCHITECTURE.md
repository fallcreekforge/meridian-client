# Architecture

Meridian Client is the public, customer-controlled half of Meridian. Proprietary Meridian Cloud
code remains outside this repository.

## Diagram maintenance

The current-state diagram describes merged code. Any PR that changes components, dependencies, or
runtime data flow must update it. The target-state diagram changes only when the intended
architecture changes.

## Current state

Arrows below are normal workspace dependencies.

```text
meridian-cli ──────────────► meridian-platform
       ├───────────────────► meridian-credential-store
       └───────────────────► meridian-utils

meridian-local ────────────► meridian-credential-store
       └───────────────────► meridian-platform

meridian-platform ─────────► meridian-credential-store
       └───────────────────► meridian-store (shared types and Ingest)

meridian-credential-store ─► meridian-utils

meridian-utils              (shared utilities; typed JSON file loading)

meridian-store              (SQLite persistence; temporary home of shared types)
```

The CLI and file credential backend use `meridian-utils::config_file` for JSON extension checks,
buffered reads, and deserialization. Callers own their schemas. The loader discards deserializer
messages so invalid secret values cannot appear in errors.

Configuration lookup uses per-user defaults, with an explicit path taking precedence:

```text
--config supplied ────────► explicit configuration file
Linux default ───────────► $XDG_CONFIG_HOME/meridian/config.json
                           (fallback: $HOME/.config/meridian/config.json)
Windows default ─────────► %LOCALAPPDATA%\Meridian\config.json

databasePath supplied ───► absolute path or path relative to configuration directory
Linux database default ──► $XDG_STATE_HOME/meridian/meridian.db
                           (fallback: $HOME/.local/state/meridian/meridian.db)
Windows database default ► %LOCALAPPDATA%\Meridian\meridian.db
```

The non-secret configuration lists games by name with explicit platforms per game.
The CLI's `status` command validates configuration and credential loading and reports the
resolved database path without opening the database, constructing a runtime, or contacting platforms.
Runtime wiring, name-to-ID resolution, and collection filtering remain planned;
`sync` and `agent` report that execution is not implemented.

`SyncEngine` polls platform-neutral `Producer` implementations. `SteamSource` owns an injected
`SteamClient` and `meridian-store::Ingest` handle, placing collection and ingestion coordination
inside the platform adapter. The store owns persistence operations. This ownership is scaffolded:
`Ingest` is a marker contract without database operations. `SteamClient` is a marker contract
without request methods; `SteamSource::poll` is unimplemented.
Scheduling, outbox coordination, ingestion, and dispatch database operations remain planned.
The Steam API reference describes upstream contracts and collection design requirements;
the client does not implement those requests or collection behavior.

```text
SyncEngine ──► Producer::poll ──► SteamSource (unimplemented collection)
                                  ├── owns steam::SteamClient marker
                                  └── owns Ingest marker
```

The database opener treats its argument as an existing filesystem path, preserving filename
characters rather than interpreting them as URL syntax. Pooled connections enforce foreign keys.
Before creating the pool, `Outbox::open` applies embedded SQL migrations on a dedicated connection.
Pending migrations and SQLx history commit atomically after a foreign-key integrity check.
Populated databases without SQLx migration history are rejected.

```text
existing filesystem Path ──► dedicated migration connection (foreign keys disabled)
                                         │
embedded migrations ──────► transaction: apply SQL + record SQLx history
                                         │
                              check foreign keys and commit
                                         │
                              close migration connection
                                         │
                                         ▼
                              Outbox pool (foreign keys enabled)
```

Development migration generation and PR checks use an isolated database:

```text
origin/main migration history ──► isolated SQLite baseline
                                         │
branch schema.sql ──► generate SQL delta ──┤
                                         ▼
                              replay delta and compare schema
                                         │
                                         ▼
                         migrations/<version>_<name>.sql + checksum
```

PR checks compare against the PR base commit, permit at most one new SQL migration, preserve
merged migration files, and verify that replay produces the declared schema. The developer's
database is independent of migration generation and verification.

Shared types live in `meridian-store::types` and are re-exported from the crate root. This is a
temporary dependency-layer placement; extraction to an independently versioned public
`meridian-types` package shared with Meridian Cloud is planned. Persistence-specific types
belong in the store. `Dispatch`, like `Ingest`, is a marker contract without database operations.

TODO: Define and test recovery when an application rollback follows a successful database
migration, including schema compatibility checks and restoration from a compatible backup.
Automatic reversal of committed migrations is unsupported.

## Target state

```text
meridian CLI
  ├── agent (primary background runtime)
  └── operator and development commands
                         │
                         ▼
                 local orchestration
                         │
CredentialStore ──► typed platform adapter
                           (Steam and others; owns an injected ingestion handle)
                         │
                         ▼
                collect / normalize / validate
                         │
                         ▼
              local SQLite transaction
       source data + watermarks + transactional outbox
                         │
                         ▼
                    outbox delivery
                         │
                         ▼
              public versioned sync protocol

════════════════════════ TRUST BOUNDARY ════════════════════════

                            Meridian Cloud
```

The shared-type dependency is separate from both product repositories:

```text
Meridian Client ──────────► meridian-types ◄────────── Meridian Cloud
```

The resident `agent` mode is the primary runtime; the other commands support setup, operation, and
development. Command invocation semantics are not an architectural constraint.

The local core remains platform-neutral and depends on the credential and platform
contracts. Outbox coordination uses the store directly. Platform adapters depend
on the store's ingestion contracts, with SQLite implementation details kept in the store.
Within the workspace, `meridian-store::types` declares the shared identifiers, supported-platform
values, and versioned envelope types, and the crate root re-exports them for platform and local
crates. The shared re-exported surface is `StudioId`, `PlatformGameId`, `Platform`,
`PlatformGame`, `ProtocolVersion`, `GameV1`, and `SyncEnvelopeV1`. Shared contracts are distinct
from persistence-specific types.

Types are shared by meaning rather than by field similarity:

| Type | Meaning and owner |
| --- | --- |
| `GameConfig` | CLI configuration: a title name and its selected platforms, before ID resolution. |
| `PlatformGame` | Shared discovery record: a resolved platform game ID and name; the producer supplies platform context. |
| `GameV1` | Shared V1 outbound allowlist: platform, resolved ID, and name. Separate from discovery records. |
| `Platform` | One shared enum used by configuration, producers, dispatch, and protocol records. |
| `StudioId`, `PlatformGameId` | Distinct shared string identifiers; studio and game identities are not interchangeable. |
| `SecretString` | The credential crate re-exports the `secrecy` type; secret file fields and client arguments use that same type. |

Shared types are defined once in `meridian-store`; the platform crate re-exports the discovery
types and `Ingest` used by its public API. Callers can implement an injected ingestion handle
without depending directly on the store. Local orchestration does not add another shared-type
re-export surface or an unused direct store dependency. Both JSON file loaders return the same
`meridian-utils::config_file::JsonFileError`; the credential crate re-exports it under that name
so callers can handle load failures without importing the utility crate. Credential lookup itself
returns a borrowed secret rather than an owned value or a file error.

An independently versioned public `meridian-types` package sits outside both `meridian-client`
and `meridian-cloud`. Both products consume it directly; database rows, query projections, and
other persistence-specific types belong in `meridian-store`.

The intended Steam financial ingestion transaction replaces every recalculated date,
advances its durable watermark, and records the outbound protocol message in the outbox atomically.
Outbox delivery can then retry independently without recollecting or losing acknowledged source
state. The public protocol explicitly limits what may cross the trust boundary. Platform
credentials have no representation in the store's outbox or in that protocol.

Schema changes use a declarative development workflow. Release artifacts apply bundled migrations
in version order. Post-release migrations are forward-only and
non-destructive so upgrading cannot invalidate existing local state.
