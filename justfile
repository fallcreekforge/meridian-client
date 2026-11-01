set shell := ["nu", "-c"]

hooks-install:
   git config --local core.hooksPath .githooks

pre-commit: fmt lint nix-fmt nix-lint toml-fmt gha-lint

fmt:
   cargo fmt --all -- --check

lint:
   cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

nix-fmt:
   nixfmt --check flake.nix

nix-lint:
   statix check flake.nix

test:
   cargo test --locked --workspace --all-features

toml-fmt:
   taplo format --check --config taplo.toml

gha-lint:
   actionlint ...(glob .github/workflows/*.yml) ...(glob .github/workflows/*.yaml)

check:
   cargo check --locked --workspace --all-targets --all-features

build:
   nix build .#meridian-client

build-windows:
   nix build .#meridian-client-windows-x86_64

# Create a development database from schema.sql, refusing to overwrite an existing file.
db-init $database_path=".local/meridian.db":
   if ($env.database_path | path exists) { error make {msg: "Database already exists; use just db-apply to update it."} }
   mkdir ($env.database_path | path expand | path dirname)
   ["BEGIN;" (open --raw crates/meridian-store/schema.sql) "PRAGMA user_version = 0;" "COMMIT;"] | str join (char newline) | sqlite3 -bail ($env.database_path | path expand)

# Inspect a database schema.
db-inspect $database_path=".local/meridian.db":
   atlas schema inspect --url $"sqlite://file:($env.database_path | path expand | url encode)"

# Print SQL that would bring a database into agreement with schema.sql.
db-diff $database_path=".local/meridian.db":
   atlas schema diff --from $"sqlite://file:($env.database_path | path expand | url encode)" --to file://crates/meridian-store/schema.sql --dev-url "sqlite://dev?mode=memory"

# Review and apply schema.sql to a development database.
db-apply $database_path=".local/meridian.db":
   atlas schema apply --url $"sqlite://file:($env.database_path | path expand | url encode)" --to file://crates/meridian-store/schema.sql --dev-url "sqlite://dev?mode=memory"

# Generate a migration from refreshed origin/main, independently of local database drift.
db-migration $name:
   nu scripts/database-migrations.nu generate $env.name

# Check the migration count, immutable history, and replay against a PR base revision.
db-migrations-check $base="origin/main":
   nu scripts/database-migrations.nu check $env.base

ci: gha-lint
   nix flake check --print-build-logs

run *args:
   cargo run --locked --package meridian-cli -- {{args}}
