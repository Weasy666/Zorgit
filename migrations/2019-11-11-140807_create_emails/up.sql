CREATE TABLE emails (
  id                INTEGER NOT NULL,
  user_id           INTEGER NOT NULL,
  address           TEXT NOT NULL,
  is_primary        BOOLEAN NOT NULL DEFAULT FALSE,
  activated_at      TIMESTAMP,
  token             TEXT,
  token_created_at  TIMESTAMP,
  notification      SMALLINTEGER NOT NULL DEFAULT 0,
  
  FOREIGN KEY (user_id) REFERENCES users(id),
  PRIMARY KEY (id)
  UNIQUE(address)
);