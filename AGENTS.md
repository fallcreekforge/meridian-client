# Repository instructions

- Use Nushell; `nix develop` starts the pinned environment.
- Meridian is the product; this repository implements Meridian Client.
- Make the smallest coherent change and avoid speculative dependencies or abstractions.
- Use tool-neutral language for development workflows in documentation and source comments;
  avoid references to paid or enterprise products.
- Describe durable behavior and contracts in documentation and comments. Avoid time-relative
  status wording such as "currently", "for now", "initial implementation", or "not yet", and
  avoid narrating the editing session. Keep progress and rollout status in dated records;
  never imply that unimplemented behavior exists.
- Use directory modules with `mod.rs` consistently. Keep `mod.rs` limited to submodule declarations
  and re-exports, and put implementations in descriptively named module files.
- Preserve the dependency graph in `ARCHITECTURE.md` and keep cloud implementation out of this
  repository.
- Update the current-state diagram in the same PR as any component, dependency, or runtime-flow
  change.
- Read `SECURITY.md` before changing credentials, protocols, transports, logging, or the
  customer-to-cloud trust boundary.
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
