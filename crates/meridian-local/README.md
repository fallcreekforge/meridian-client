# meridian-local

Owns reusable customer-side synchronization orchestration.

`SyncEngine` polls platform-neutral producers and supplies local credential access. Platform
adapters own their typed clients and injected ingestion handles; they coordinate collection and
ingestion while `meridian-store` owns persistence operations and transaction mechanics. Local
orchestration does not own platform-specific queries, database implementation, cloud transport,
or CLI concerns. Scheduling and outbox coordination remain planned.
