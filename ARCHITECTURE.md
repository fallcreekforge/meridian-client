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
meridian-cli                 (standalone command parsing and presentation)

meridian-local ────────────► meridian-credential-store
       ├───────────────────► meridian-platform
       └───────────────────► meridian-store

meridian-platform ─────────► meridian-credential-store

meridian-credential-store  (studio-controlled credential access)

meridian-store              (SQLx-backed SQLite persistence boundary)
```

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

The resident `agent` mode is the primary runtime; the other commands support setup, operation, and
development. Command invocation semantics are not an architectural constraint.

The local core remains platform-neutral and consumes `meridian-store` for durable state. For
Steam financial collection, a completed transaction replaces every recalculated date, advances
its durable watermark, and records the outbound protocol message in the outbox atomically. Outbox
delivery can then retry independently without recollecting or losing acknowledged source state.
Platform crates implement typed capabilities, and the public protocol explicitly limits what may
cross the trust boundary. Platform credentials have no representation in the store's outbox or in
that protocol.

Atlas manages schema changes during development. Release artifacts apply bundled, ordered
migrations in a Flyway-style sequence. Post-release migrations are forward-only and
non-destructive so upgrading cannot invalidate existing local state.
