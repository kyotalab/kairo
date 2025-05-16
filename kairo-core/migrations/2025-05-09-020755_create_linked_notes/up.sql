-- Your SQL goes here
CREATE TABLE linked_notes (
    id TEXT NOT NULL PRIMARY KEY,
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    link_type TEXT,
    created_at TIMESTAMP NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);