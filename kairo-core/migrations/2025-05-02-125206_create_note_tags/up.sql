-- Your SQL goes here
CREATE TABLE note_tags (
    note_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (note_id, tag_id),
    FOREIGN KEY(note_id) REFERENCES notes(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);