CREATE TABLE labels (
  id          INTEGER NOT NULL,
  project_id  INTEGER NOT NULL,
  name        TEXT NOT NULL,
  description TEXT,
  color       TEXT NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id),
  PRIMARY KEY (id)
);

CREATE TABLE issues_labels (
  id            INTEGER NOT NULL,
  label_id      INTEGER NOT NULL,
  issue_id      INTEGER NOT NULL,

  FOREIGN KEY (label_id) REFERENCES labels(id),
  FOREIGN KEY (issue_id) REFERENCES issues(id)
  PRIMARY KEY (id),
  UNIQUE(label_id, issue_id)
);