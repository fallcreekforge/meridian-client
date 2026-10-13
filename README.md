# Meridian Client

Meridian is a Fall Creek Forge product for game studios. Meridian Client is its
open-source software for studio-controlled infrastructure.

Meridian Client is the customer-controlled runtime responsible for keeping platform credentials in
studio infrastructure, collecting and normalizing platform data, persisting local operational
state, and delivering an explicit, auditable payload to Meridian Cloud. Its `agent` command is the
background runtime entry point; other commands support setup, operation, and development.

## Development

The pinned Fenix environment opens Nushell and provides the project tools:

```nu
nix develop
just --list
just ci
```

Nix owns package builds. Build or run the native Linux binary with:

```nu
just build
nix run .#meridian-client -- --version
```

## Continuous integration

The flake is the authoritative CI definition. Run `just ci` before handoff. CI agent and
binary-cache credentials remain outside this repository.

See [ARCHITECTURE.md](ARCHITECTURE.md), [SECURITY.md](SECURITY.md), and
[CONTRIBUTING.md](CONTRIBUTING.md) for repository-specific details.
Cross-repository product documentation lives in the
[`fallcreekforge-docs`](fallcreekforge-docs/meridian/meridian-client) submodule.

## License

Licensed under the [Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for attribution.
