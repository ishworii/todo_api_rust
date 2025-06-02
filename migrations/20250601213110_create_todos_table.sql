-- Add migration script here
CREATE TABLE IF NOT EXISTS todos(
  id TEXT PRIMARY KEY NOT NULL,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL
);
