CREATE TABLE mother_tongue (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  is_vetted BOOLEAN NOT NULL DEFAULT FALSE,
  meta TEXT NOT NULL
)
