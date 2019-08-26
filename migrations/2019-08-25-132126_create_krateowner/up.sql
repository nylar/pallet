CREATE TABLE krateowner (
  krate_id INTEGER NOT NULL,
  owner_id INTEGER NOT NULL,
  PRIMARY KEY (krate_id, owner_id),
  foreign key (krate_id) references krate(id),
  foreign key (owner_id) references owner(id)
);
