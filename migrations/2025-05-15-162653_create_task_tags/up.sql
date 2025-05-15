-- Your SQL goes here
CREATE TABLE task_tags (
    task_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY(task_id) REFERENCES tasks(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);