CREATE TABLE attachments (
  id                INTEGER NOT NULL,
  uuid              TEXT NOT NULL,
  user_id           INTEGER NOT NULL,
  project_id        INTEGER,
  issue_id          INTEGER,
  comment_id        INTEGER,
  name              TEXT NOT NULL,
  download_count    INTEGER NOT NULL DEFAULT 0,
  
  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (issue_id) REFERENCES issues(id),
  FOREIGN KEY (comment_id) REFERENCES comments(id),
  PRIMARY KEY (id)
  UNIQUE(uuid)
);