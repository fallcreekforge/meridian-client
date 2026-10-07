# meridian-store

- Keep SQLite-specific persistence, row and query projection types, transactions, and migrations
  in this crate.
- Keep this crate independent of CLI, platform, credential, and cloud implementations.
- Keep common persistence contracts platform-neutral; isolate platform-specific records instead of
  treating one platform's workflow as the whole store.
- Commit related state changes and outbound messages atomically through the transactional outbox.
- Keep post-release migrations forward-only and non-destructive.
- Keep database rows and query projections here; storage-neutral domain types belong outside the
  persistence layer.
