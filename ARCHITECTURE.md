# Architecture

Meridian Client is the public, customer-controlled half of Meridian. Proprietary Meridian Cloud
code remains outside this repository.

## Diagram maintenance

The current-state diagram describes merged code. Any PR that changes components, dependencies, or
runtime data flow must update it. The target-state diagram changes only when the intended
architecture changes.

## Current state

Arrows below are normal workspace dependencies; the dashed arrow is test-only.

```text
meridian-cli                 (standalone command parsing)

meridian-local ────────────► meridian-credential-store
       ├───────────────────► meridian-platform
       └───────────────────► meridian-types

meridian-platform ─────────► meridian-credential-store
       └───────────────────► meridian-types

meridian-credential-store
       └── FileCredentialStore (customer-controlled configuration)
```

`SyncEngine` owns and can produce a versioned envelope through injected credential and platform
contracts. `meridian-platform` includes the typed Steam adapter alongside the platform-neutral
contract.
`FileCredentialStore` can hold credentials deserialized from customer-controlled configuration.
The CLI is not connected to either implementation, and no real platform or cloud request is
implemented.

## Target state

```text
meridian CLI ── local IPC ──► meridian-agent
                                  │
CredentialStore ──► typed platform adapter
                           (Steam and others)
                                  │
                                  ▼
                              SyncEngine
                                  │
                       normalize / filter / validate
                                  │
                                  ▼
                     public versioned sync protocol

════════════════════════ TRUST BOUNDARY ════════════════════════

                            Meridian Cloud
```

The local core remains platform-neutral. Platform crates implement typed capabilities, and the
public protocol explicitly limits what may cross the trust boundary. Platform credentials have no
representation in that protocol.
