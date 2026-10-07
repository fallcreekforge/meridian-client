set shell := ["nu", "-c"]

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
   actionlint .github/workflows/windows.yml

check:
   cargo check --locked --workspace --all-targets --all-features

build:
   nix build .#meridian-client

build-windows:
   nix build .#meridian-client-windows-x86_64

ci: gha-lint
   nix flake check --print-build-logs

run *args:
   cargo run --locked --package meridian-cli -- {{args}}
