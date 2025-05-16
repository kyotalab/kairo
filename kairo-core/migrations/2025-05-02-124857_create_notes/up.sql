-- Your SQL goes here
CREATE TABLE notes (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    note_type TEXT NOT NULL,
    sub_type TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    project_id TEXT,
    task_id TEXT
);