# Repository instructions

- Use Nushell; `nix develop` starts the pinned environment.
- Meridian is the product; this repository implements Meridian Client.
- Make the smallest coherent change and avoid speculative dependencies or abstractions.
- Use directory modules with `mod.rs` consistently. Keep `mod.rs` limited to submodule declarations
  and re-exports, and put implementations in descriptively named module files.
- Preserve the dependency graph in `ARCHITECTURE.md` and keep cloud implementation out of this
  repository.
- Update the current-state diagram in the same PR as any component, dependency, or runtime-flow
  change.
- Keep cross-repository product and domain decisions in `fallcreekforge-docs`.
- For external documentation, read the relevant context in the `fallcreekforge-docs` submodule
  when available.
- Read `SECURITY.md` before changing credentials, protocols, transports, logging, or the
  customer-to-cloud trust boundary.
- Update an ADR in `fallcreekforge-docs/meridian/meridian-client/adr` only when an architectural
  contract changes.
- Follow crate-local `AGENTS.md` files for boundary-specific rules.

During development, run the smallest checks relevant to the change.

Before declaring work merge-ready, run:

```nu
just fmt
just lint
just test
```

Also run `just ci` before a merge-ready handoff when changing dependencies, `Cargo.lock`, Nix or
build configuration, workflows, feature flags, target-specific code, or broad cross-crate
contracts, or when explicitly requested.

If full CI was not run, state that clearly at handoff.
