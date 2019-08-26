CREATE TABLE token (
  id SERIAL PRIMARY KEY,
  owner_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  api_token TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,

  foreign key (owner_id) references owner(id)
)
