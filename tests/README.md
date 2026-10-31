# Repository end-to-end tests

Keep repository-wide end-to-end scenarios and their synthetic fixture scripts here. Add SQL
data fixtures alongside the tests that need them; use the canonical schema in
[`crates/meridian-store/schema.sql`](../crates/meridian-store/schema.sql) to create test databases.

Each database test should use an isolated temporary database and clean it up afterward. Fixtures
must contain only synthetic data and credentials.

The workspace root is a virtual Cargo manifest, so Rust tests here are not automatically
discovered. Register each test target in the appropriate crate manifest with an explicit path
when adding an executable test. Crate-specific tests can continue to live with their crate.

Database connection and transaction tests belong to `meridian-store`, alongside its SQL fixtures.
