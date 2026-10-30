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
       ├───────────────────► meridian-platform
       └───────────────────► meridian-store

meridian-platform ─────────► meridian-credential-store
       └───────────────────► meridian-store

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
                           (Steam and others)
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

The local core remains platform-neutral and consumes `meridian-store` for durable state. In the
current workspace, `meridian-store::types` declares the shared identifiers, supported-platform
values, and versioned envelope types, and the crate root re-exports them for platform and local
crates. The current re-exported surface is `StudioId`, `PlatformGameId`, `Platform`,
`PlatformGame`, `ProtocolVersion`, `GameV1`, and `SyncEnvelopeV1`. This is a temporary placement in
the lowest existing dependency layer, not persistence ownership of those contracts.

The target is an independently versioned public `meridian-types` package outside both
`meridian-client` and `meridian-cloud`. Both products will consume it directly; database rows,
query projections, and other persistence-specific types will remain in `meridian-store`. Until
Meridian Cloud development begins and that package is extracted, client crates consume the shared
types through the `meridian-store` re-exports.

For Steam financial collection, a completed transaction replaces every recalculated date,
advances its durable watermark, and records the outbound protocol message in the outbox atomically.
Outbox delivery can then retry independently without recollecting or losing acknowledged source
state. The public protocol explicitly limits what may cross the trust boundary. Platform
credentials have no representation in the store's outbox or in that protocol.

Atlas manages schema changes during development. Release artifacts apply bundled, ordered
migrations in a Flyway-style sequence. Post-release migrations are forward-only and
non-destructive so upgrading cannot invalidate existing local state.
