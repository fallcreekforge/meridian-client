# meridian-cli

Builds the `meridian` command-line interface.

The crate owns command parsing and presentation. `agent` is the background runtime entry point;
operator and development commands use the same reusable behavior from `meridian-local`.
