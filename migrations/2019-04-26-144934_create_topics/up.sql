CREATE TABLE topics (
  id        INTEGER NOT NULL,
  name      TEXT NOT NULL,

  PRIMARY KEY (id),
  UNIQUE(name)
);

CREATE TABLE project_topics (
  id            INTEGER NOT NULL,
  topic_id      INTEGER NOT NULL,
  project_id    INTEGER NOT NULL,

  FOREIGN KEY (topic_id) REFERENCES topics(id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
  PRIMARY KEY (id),
  UNIQUE(topic_id, project_id)
);