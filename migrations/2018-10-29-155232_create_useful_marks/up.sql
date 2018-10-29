CREATE TABLE useful_marks (
  card_id INTEGER NOT NULL REFERENCES cards(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (card_id, user_id)
);
CREATE INDEX ON useful_marks(card_id);
CREATE INDEX ON useful_marks(user_id);
