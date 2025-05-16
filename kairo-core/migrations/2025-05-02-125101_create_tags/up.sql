-- Your SQL goes here
CREATE TABLE tags (
    id TEXT NOT NULL PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);