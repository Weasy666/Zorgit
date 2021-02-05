CREATE TABLE users (
  id                INTEGER PRIMARY KEY NOT NULL,
  types              SMALLINT NOT NULL,
  username          TEXT NOT NULL UNIQUE,
  full_name         TEXT,
  avatar            TEXT NOT NULL,
  avatar_email      TEXT,
  password          TEXT NOT NULL,
  salt              TEXT NOT NULL,
  location          TEXT,
  website           TEXT,
  description       TEXT,
  language          TEXT(5) NOT NULL DEFAULT 'en-EN',

  must_change_password  BOOLEAN NOT NULL DEFAULT false,
  is_email_hidden       BOOLEAN NOT NULL DEFAULT false,
  is_admin              BOOLEAN NOT NULL DEFAULT false,
  is_organisation       BOOLEAN NOT NULL DEFAULT false,

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_seen_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);