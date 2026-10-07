# meridian-local

Owns reusable customer-side synchronization orchestration.

Platform collection, credential access, and durable state are consumed through their crate
boundaries. This crate normalizes collected data into the public versioned sync protocol without
owning platform-specific, persistence, cloud-transport, or CLI concerns.
