CREATE TABLE issues (
  id            INTEGER NOT NULL,
  number        INTEGER NOT NULL,
  project_id    INTEGER NOT NULL,
  user_id       INTEGER NOT NULL,
  title         TEXT NOT NULL,
  content       TEXT NOT NULL,
  num_comments  INTEGER NOT NULL DEFAULT 0,
  is_closed     BOOLEAN NOT NULL DEFAULT false,
  
  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  PRIMARY KEY (id)
);