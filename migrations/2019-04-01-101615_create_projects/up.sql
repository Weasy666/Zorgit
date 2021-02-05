CREATE TABLE projects (
  id              INTEGER PRIMARY KEY NOT NULL,
  user_id         INTEGER NOT NULL,
  name            TEXT NOT NULL,
  description     TEXT,
  website         TEXT,
  default_branch  TEXT,

  num_watches           INTEGER NOT NULL DEFAULT 1, -- because at least the owner is watching
  num_stars             INTEGER NOT NULL DEFAULT 0,
  num_forks             INTEGER NOT NULL DEFAULT 0,
  num_issues            INTEGER NOT NULL DEFAULT 0,
  num_issues_closed     INTEGER NOT NULL DEFAULT 0,
  num_issues_open       INTEGER NOT NULL DEFAULT 0,
  num_labels            INTEGER NOT NULL DEFAULT 0,
  num_pull_reqs         INTEGER NOT NULL DEFAULT 0,
  num_pull_reqs_closed  INTEGER NOT NULL DEFAULT 0,
  num_pull_reqs_open    INTEGER NOT NULL DEFAULT 0,
  num_milestones        INTEGER NOT NULL DEFAULT 0,
  num_milestones_closed INTEGER NOT NULL DEFAULT 0,
  num_milestones_open   INTEGER NOT NULL DEFAULT 0,
  num_releases          INTEGER NOT NULL DEFAULT 0,

  is_private    BOOLEAN NOT NULL DEFAULT true,
  is_empty      BOOLEAN NOT NULL DEFAULT true,
  is_archived   BOOLEAN NOT NULL DEFAULT false,

  vcs             INTEGER NOT NULL DEFAULT 0,
  is_fork         BOOLEAN NOT NULL DEFAULT false,
  forked_project  INTEGER REFERENCES projects(id),
  disk_size       UNSIGNED BIG INT NOT NULL DEFAULT 0,

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (user_id) REFERENCES users(id),
  UNIQUE(user_id, name)
);

-- collaborators of a project
CREATE TABLE projects_users (
  id          INTEGER NOT NULL,
  project_id  INTEGER NOT NULL,
  user_id     INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
  PRIMARY KEY (id),
  UNIQUE(project_id, user_id)
);