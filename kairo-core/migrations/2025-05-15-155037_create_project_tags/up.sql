-- Your SQL goes here
CREATE TABLE project_tags (
    project_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (project_id, tag_id),
    FOREIGN KEY(project_id) REFERENCES projects(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);