set shell := ["nu", "-c"]

# Enable repository Git hooks for this checkout.
hooks-install:
   git config --local core.hooksPath .githooks

# Run formatting and lint checks used by the commit hook.
pre-commit: fmt lint nix-fmt nix-lint toml-fmt gha-lint

# Check Rust formatting without changing files.
fmt:
   cargo fmt --all -- --check

# Lint all Rust targets and features, treating warnings as errors.
lint:
   cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

# Check Nix formatting without changing files.
nix-fmt:
   nixfmt --check flake.nix

# Lint the Nix build and development configuration.
nix-lint:
   statix check flake.nix

# Run workspace tests with all features enabled.
test:
   cargo test --locked --workspace --all-features

# Check TOML formatting without changing files.
toml-fmt:
   taplo format --check --config taplo.toml

# Validate GitHub Actions workflows.
gha-lint:
   actionlint ...(glob .github/workflows/*.yml) ...(glob .github/workflows/*.yaml)

# Type-check all Rust targets and features without building executables.
check:
   cargo check --locked --workspace --all-targets --all-features

# Build the Linux release package.
build:
   nix build .#meridian-client

# Cross-compile the Windows x86-64 GNU release package.
build-windows:
   nix build .#meridian-client-windows-x86_64

# Create a scratch database from schema.sql without SQLx migration history; refuse overwrites.
db-init $database_path=".local/meridian.db":
   if ($env.database_path | path exists) { error make {msg: "Database already exists; use just db-apply to update it."} }
   mkdir ($env.database_path | path expand | path dirname)
   ["BEGIN;" (open --raw crates/meridian-store/schema.sql) "PRAGMA user_version = 0;" "COMMIT;"] | str join (char newline) | sqlite3 -bail ($env.database_path | path expand)

# Inspect a database schema.
db-inspect $database_path=".local/meridian.db":
   atlas schema inspect --url $"sqlite://file:($env.database_path | path expand | url encode)"

# Print SQL needed to match schema.sql without applying changes.
db-diff $database_path=".local/meridian.db":
   atlas schema diff --from $"sqlite://file:($env.database_path | path expand | url encode)" --to file://crates/meridian-store/schema.sql --dev-url "sqlite://dev?mode=memory"

# Preview and confirm changes to match schema.sql without updating migration history.
db-apply $database_path=".local/meridian.db":
   atlas schema apply --url $"sqlite://file:($env.database_path | path expand | url encode)" --to file://crates/meridian-store/schema.sql --dev-url "sqlite://dev?mode=memory"

# Generate a migration from refreshed origin/main, independently of local database drift.
db-migration $name:
   nu scripts/database-migrations.nu generate $env.name

# Check the migration count, immutable history, and replay against a PR base revision.
db-migrations-check $base="origin/main":
   nu scripts/database-migrations.nu check $env.base

# Run workflow validation and all Nix CI checks.
ci: gha-lint
   nix flake check --print-build-logs

# Run the CLI with the supplied arguments.
run *args:
   cargo run --locked --package meridian-cli -- {{args}}
