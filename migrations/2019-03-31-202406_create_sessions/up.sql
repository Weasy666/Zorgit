CREATE TABLE sessions (
  id          INTEGER PRIMARY KEY NOT NULL,
  user_id     INTEGER NOT NULL,
  token       TEXT NOT NULL,
  created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires     TIMESTAMP NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(id)
);