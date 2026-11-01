-- Synthetic tables for connection and transaction tests.
CREATE TABLE test_parent (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE test_child (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER NOT NULL REFERENCES test_parent (id) ON DELETE CASCADE
);
