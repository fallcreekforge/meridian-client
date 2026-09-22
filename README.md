# Meridian Client

Meridian is a Fall Creek Forge product for independent game studios. Meridian Client is its
open-source software for studio-controlled infrastructure.

Meridian Client keeps platform credentials local and produces an explicit, auditable payload for
Meridian Cloud. The current implementation defines the core types, interfaces, and local
orchestration. No platform or cloud network client is implemented.

## Development

The pinned Fenix environment opens Nushell and provides the project tools:

```nu
nix develop
just ci
just run --version
just run status
just run sync
```

`status` and `sync` report the current unconfigured state.

Nix owns package builds. Build or run the native Linux binary with:

```nu
just build
nix run .#meridian-client -- --version
```

The initial Windows output is an x86-64 GNU portability build produced on Linux:

```nu
just build-windows
```

It verifies that the workspace cross-compiles to `meridian.exe`; it is not yet a supported release
artifact or a decision about the eventual Windows ABI, installer, or signing process.

## Continuous integration

The flake is the authoritative CI definition. Hercules CI builds the Linux package, verifies stable
Rust compatibility, cross-builds the Windows package, and runs repository checks on its
`x86_64-linux` agent. A GitHub Actions job separately builds, tests, and runs Meridian Client
natively on Windows with the MSVC toolchain. Hercules CI agent and binary-cache credentials remain
outside this repository.

See [ARCHITECTURE.md](ARCHITECTURE.md), [SECURITY.md](SECURITY.md), and
[CONTRIBUTING.md](CONTRIBUTING.md) for repository-specific details.

## License

Licensed under the [Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for attribution.
