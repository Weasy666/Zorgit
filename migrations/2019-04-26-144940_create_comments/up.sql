CREATE TABLE comments (
  id            INTEGER NOT NULL,
  enum_type     SMALLINT NOT NULL,
  issue_id      INTEGER NOT NULL,
  user_id       INTEGER NOT NULL,
  content       TEXT NOT NULL,
  
  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (issue_id) REFERENCES issues(id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  PRIMARY KEY (id)
);