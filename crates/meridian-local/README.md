# meridian-local

Contains reusable customer-side synchronization logic.

`SyncEngine` collects games through the platform-neutral `PlatformClient` contract and maps them
into its explicit, versioned sync payload types. Platform and credential implementations can change
without changing orchestration. The CLI is not connected yet.
