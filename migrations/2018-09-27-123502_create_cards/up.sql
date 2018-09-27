CREATE TABLE cards (
  id SERIAL PRIMARY KEY,
  author_id INTEGER NOT NULL REFERENCES users(id),
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX index_author_id ON cards(author_id);
