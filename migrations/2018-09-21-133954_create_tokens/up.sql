CREATE TABLE tokens (
  token VARCHAR PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id)
);
CREATE INDEX index_user_id ON tokens(user_id);
