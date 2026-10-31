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
The non-secret configuration lists games by name with explicit platforms per game. The CLI's
`status` command validates configuration and credential loading without constructing a runtime or
contacting platforms. `--version` and `-V` exit before loading either file. Runtime wiring,
name-to-ID resolution, and collection filtering remain planned; `sync` and `agent` report that
execution is not implemented.

`SyncEngine` polls platform-neutral `Producer` implementations. `SteamSource` owns an injected
`SteamClient` and `meridian-store::Ingest` handle, placing collection and ingestion coordination
inside the platform adapter. The store owns persistence operations. This ownership is scaffolded:
`Ingest` has no operations yet, and the Steam polling path does not yet persist collected data.

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

The planned shared-type dependency is separate from both product repositories:

```text
Meridian Client ──────────► meridian-types ◄────────── Meridian Cloud
```

The resident `agent` mode is the primary runtime; the other commands support setup, operation, and
development. Command invocation semantics are not an architectural constraint.

The local core remains platform-neutral and currently depends on the credential and platform
contracts. Its planned outbox coordination will use the store directly. Platform adapters depend
on the store's ingestion contracts, with SQLite implementation details kept in the store.
In the current workspace, `meridian-store::types` declares the shared identifiers, supported-platform
values, and versioned envelope types, and the crate root re-exports them for platform and local
crates. The current re-exported surface is `StudioId`, `PlatformGameId`, `Platform`,
`PlatformGame`, `ProtocolVersion`, `GameV1`, and `SyncEnvelopeV1`. This is a temporary placement in
the lowest existing dependency layer, not persistence ownership of those contracts.

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

The target is an independently versioned public `meridian-types` package outside both
`meridian-client` and `meridian-cloud`. Both products will consume it directly; database rows,
query projections, and other persistence-specific types will remain in `meridian-store`. Until
Meridian Cloud development begins and that package is extracted, client crates consume the shared
types through the `meridian-store` re-exports.

The intended Steam financial ingestion transaction replaces every recalculated date,
advances its durable watermark, and records the outbound protocol message in the outbox atomically.
Outbox delivery can then retry independently without recollecting or losing acknowledged source
state. The public protocol explicitly limits what may cross the trust boundary. Platform
credentials have no representation in the store's outbox or in that protocol.

Atlas manages schema changes during development. Release artifacts apply bundled, ordered
migrations in a Flyway-style sequence. Post-release migrations are forward-only and
non-destructive so upgrading cannot invalidate existing local state.
