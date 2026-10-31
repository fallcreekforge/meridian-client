# Security

## Reporting

Do not open a public issue for a suspected vulnerability. Contact Fall Creek Forge privately and
never include live credentials or customer data in a report.

## Trust boundary

- Platform credentials remain in studio-controlled infrastructure.
- Meridian Cloud receives only values represented by the public versioned sync protocol.
- Passwords, sessions, Steam Guard secrets, API keys, and arbitrary credential-store values must
  never enter that protocol.

Unknown protocol fields are rejected. Any protocol change that expands what can cross the boundary
requires security review.

## Local persistence

The SQLite store remains studio-controlled and may contain sensitive financial data.
Implementations must restrict access to the database and its backups using operating-system file
permissions appropriate to credential-adjacent application data.

Transactional outbox records may contain only versioned sync-protocol payloads and delivery
metadata. They must not contain platform credentials, arbitrary credential-store values, or
unfiltered platform responses.

## Development rules

- Never log credentials or include them in errors.
- Never commit real credentials in code, fixtures, examples, or snapshots.
- Keep test credentials unmistakably synthetic.
- Keep platform clients typed; do not add a generic endpoint escape hatch.
