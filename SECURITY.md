# Security

## Reporting

Do not open a public issue for a suspected vulnerability. Contact Fall Creek Forge privately and
never include live credentials or customer data in a report.

## Trust boundary

- Platform credentials remain in studio-controlled infrastructure.
- Meridian Cloud receives only values represented by `meridian-local`'s versioned sync payload
  types.
- Passwords, sessions, Steam Guard secrets, API keys, and arbitrary credential-store values must
  never enter that protocol.

Unknown protocol fields are rejected. Any protocol change that expands what can cross the boundary
requires security review.

## Development rules

- Never log credentials or include them in errors.
- Never commit real credentials in code, fixtures, examples, or snapshots.
- Keep test credentials unmistakably synthetic.
- Keep platform clients typed; do not add a generic endpoint escape hatch.
