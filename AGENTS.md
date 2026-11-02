# Repository instructions

- Use Nushell; `nix develop` starts the pinned environment.
- Meridian is the product; this repository implements Meridian Client.
- Make the smallest coherent change and avoid speculative dependencies or abstractions.
- Add `#[must_use]` only when Clippy requests it; do not add it routinely to constructors or accessors.
- Prefer upstream examples in API documentation; reserve synthetic examples for clearly labeled edge cases.
- Use tool-neutral language for development workflows in documentation and source comments;
  avoid references to paid or enterprise products.
- Describe durable behavior and contracts in documentation and comments. Reserve implementation
  status, gaps, and temporary arrangements for the Current state section of `ARCHITECTURE.md`.
  Outside that section, avoid time-relative wording such as "currently", "for now",
  "initial implementation", or "not yet", and avoid narrating the editing session.
  Keep progress and rollout status in dated records; never imply that unimplemented behavior exists.
- Follow idiomatic Rust module organization. When a module uses a directory, prefer
  `<module>/mod.rs` over a sibling `<module>.rs`. `mod.rs` may contain types, traits,
  implementations, and tests; split code into focused files when it improves readability.
- Keep inherent `impl` blocks beside their type definitions. Put supporting functions in helper
  modules rather than moving a type's methods into an unrelated file.
- Prefer inline `#[cfg(test)] mod tests` beside the implementation for small unit-test suites.
  Use separate test modules for larger suites, such as database tests. Cargo integration tests
  belong in `tests/` as separate test targets.
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
