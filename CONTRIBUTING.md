# Contributing

Enter the pinned Nushell environment and run the repository gate:

```nu
nix develop
just ci
```

The development shell includes the SQLite CLI (`sqlite3`) for inspecting local databases and
trying schema changes.

Entering the development shell installs Git hooks through git-hooks.nix. Hook definitions and
tool versions come from `flake.nix` and `flake.lock`; `.pre-commit-config.yaml` is generated
and ignored by Git. The installed hooks use Nix store executables without entering a new shell
for each commit.

At commit time, pre-commit selects hooks by the staged file paths and temporarily sets aside
unstaged edits to tracked files. Formatters apply Rust, Nix, TOML, and SQL fixes; Clippy,
Statix, and actionlint report errors. Rust formatting and Clippy operate on the workspace.
If a formatter changes files, the hook stops so you can review and stage the changes before
retrying the commit. Hooks do not stage files automatically.

Run `just pre-commit` to run all hooks across the working tree, including unstaged changes.
`just ci` runs the same hook definitions against the whole repository in the Nix build sandbox,
alongside tests and package builds. Git hooks provide local feedback; CI enforces the repository
gate independently.

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
