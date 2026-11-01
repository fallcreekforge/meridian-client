# meridian-cli

Builds the `meridian` command-line interface.

The crate owns command parsing and presentation. `agent` is the background runtime entry point;
operator and development commands use the same reusable behavior from `meridian-local`.

Pass `--config /path/to/config.json` to select a JSON configuration file. See
[`sample/config.json`](../../sample/config.json) for an example. Each entry in `games` has a
human-readable `name` and an explicit list of `platforms` on which to collect that game (currently
only `steam` is supported). Platforms omitted from a game are outside its collection scope.
This replaces the former platform maps and `allowedGames` lists. Unknown platforms and unknown
configuration fields are rejected. `credentialsFile` points to a separate JSON credential file;
relative paths are resolved from the process working directory.

`databasePath` is optional and selects the SQLite file. Absolute paths support persistent volumes;
relative paths are resolved against the configuration file's directory. An empty path is rejected.
Omitting it selects the per-user SQLite default below. If no default is available, set
`databasePath` explicitly.

`status` checks that configuration and credentials can be loaded; it does not check platform
connectivity or whether an agent is running. It reports the resolved database path without opening
or creating the database or its parent directories. `sync` and `agent` return an explicit
not-implemented error. Local name-to-ID resolution and per-game collection filtering remain planned.

`--version` (or `-V`) prints the version without requiring configuration or credentials.

| Default | Windows (per user) | Linux (per user) |
| --- | --- | --- |
| Configuration | `%LOCALAPPDATA%\Meridian\config.json` | `$XDG_CONFIG_HOME/meridian/config.json` (fallback: `$HOME/.config/meridian/config.json`) |
| SQLite state | `%LOCALAPPDATA%\Meridian\state.db` | `$XDG_STATE_HOME/meridian/state.db` (fallback: `$HOME/.local/state/meridian/state.db`) |

Default paths are relative to the corresponding user directory. Path resolution uses
`LOCALAPPDATA` on Windows, or `XDG_CONFIG_HOME` and `XDG_STATE_HOME` with the `HOME` fallbacks on Linux.
Unset, empty, or relative XDG directory values use the fallback; a usable home directory must
also be absolute. No literal `~` is passed to filesystem APIs. If the default location is
unavailable, pass `--config` explicitly. Both platforms use per-user defaults; a shared service
installation would need a separately chosen machine-wide location.

From the repository root, run `meridian status --config sample/config.json` to use the sample
configuration and synthetic credentials in `sample/secrets.json`. These are smoke-test fixtures;
configure a private credential file for actual use.

The credential file requires `meridianCloudKey`, `steamFinancialWebApiKey`, and
`steamPublisherWebApiKey` string fields. Keep real credentials out of ordinary configuration.
Both files use the shared loader in `meridian-utils::config_file`. Errors report file access or
JSON/schema failure without echoing file contents. Commands are still under development.
