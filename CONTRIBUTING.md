# Contributing

Enter the pinned Nushell environment and run the repository gate:

```nu
nix develop
just ci
```

`just ci` runs the repository gate defined by the flake. Keep CI agent, cache, and signing
credentials out of the repository.

The workspace's minimum supported Rust version tracks the latest stable release line. Development,
linting, testing, and package builds use stable Rust pinned by Fenix. Formatting uses the pinned
nightly `rustfmt` required by `rustfmt.toml`. When updating toolchains, update
`workspace.package.rust-version` in the same change.

Keep changes small and preserve the dependency direction in [ARCHITECTURE.md](ARCHITECTURE.md).
Update its current-state diagram when a PR changes components, dependencies, or runtime data flow.
Protocol expansions require security review. Architectural contract changes require a concise ADR
in `fallcreekforge-docs/meridian/meridian-client/adr`.
Never use real credentials in development or tests.
