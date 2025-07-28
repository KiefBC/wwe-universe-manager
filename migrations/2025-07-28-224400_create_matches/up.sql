CREATE TABLE matches (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  show_id INTEGER NOT NULL REFERENCES shows(id) ON DELETE CASCADE,
  match_name VARCHAR,
  match_type VARCHAR NOT NULL DEFAULT 'Singles',
  match_stipulation VARCHAR,
  scheduled_date DATE,
  match_order INTEGER,
  winner_id INTEGER REFERENCES wrestlers(id),
  is_title_match BOOLEAN NOT NULL DEFAULT FALSE,
  title_id INTEGER REFERENCES titles(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);