CREATE TABLE version (
  id SERIAL PRIMARY KEY,
  krate_id INTEGER NOT NULL,
  vers TEXT NOT NULL,
  yanked BOOL DEFAULT false NOT NULL,
  unique(vers, krate_id),
  foreign key (krate_id) references krate(id)
)
