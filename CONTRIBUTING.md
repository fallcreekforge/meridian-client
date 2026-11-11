# Contributing

Enter the pinned Nushell environment and run the repository gate:

```nu
nix develop
just ci
```

The development shell includes the SQLite CLI (`sqlite3`) for inspecting local databases and
trying schema changes.

Entering the development shell from the repository root enables the tracked Git hooks in
`.githooks`. An existing custom hooks path is preserved; run `just hooks-install` to select
the repository hooks explicitly.

The pre-commit hook enters the pinned development environment and runs `just pre-commit`.
It first applies automatic SQLFluff fixes to standalone SQL in the working tree, including
unstaged changes. If any files change, the hook stops so you can review and stage them before
retrying the commit. The hook does not stage files automatically.
When SQL needs no fixes, it checks Rust formatting and Clippy, Nix formatting and linting,
TOML formatting, SQL linting, and workflow linting without rewriting files.
Run `just pre-commit` to run this workflow manually. Tests and package builds run through `just ci`.
Git hooks provide local feedback; CI enforces the repository gate independently.

SQLFluff checks standalone SQL files with the SQLite dialect. Align column data types within
each table, use four-space indentation, and use single spaces elsewhere. SQLFluff enforces
the alignment through its layout configuration. Run `just sql-lint` to check files and
`just sql-fix` to apply automatic formatting and lint fixes. SQL embedded in Rust
strings is outside this check. Atlas-generated migrations are excluded to preserve their
checksums and immutable history; migration policy and replay have separate checks.

`just ci` runs the repository gate defined by the flake. Keep CI agent, cache, and signing
credentials out of the repository.

The workspace's minimum supported Rust version tracks the latest stable release line. Development,
linting, testing, and package builds use stable Rust pinned by Fenix. Formatting uses the pinned
nightly `rustfmt` required by `rustfmt.toml`. When updating toolchains, update
`workspace.package.rust-version` in the same change.

Keep changes small and preserve the dependency direction in [ARCHITECTURE.md](ARCHITECTURE.md).
Update its current-state diagram when a PR changes components, dependencies, or runtime data flow.
Protocol expansions require security review.
Never use real credentials in development or tests.
