-- CREATE TABLE assignees (
--   id            INTEGER NOT NULL,
--   user_id       INTEGER NOT NULL,
--   issue_id      INTEGER NOT NULL,
--   project_id       INTEGER NOT NULL,

--   FOREIGN KEY (user_id) REFERENCES users(id),
--   FOREIGN KEY (issue_id) REFERENCES issues(id),
--   FOREIGN KEY (project_id) REFERENCES projects(id),
--   PRIMARY KEY (id)
-- );

-- assignees
CREATE TABLE issues_users (
  id            INTEGER NOT NULL,
  user_id       INTEGER NOT NULL,
  issue_id      INTEGER NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (issue_id) REFERENCES issues(id)
  PRIMARY KEY (id),
  UNIQUE(user_id, issue_id)
);