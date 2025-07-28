CREATE TABLE match_participants (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  match_id INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
  wrestler_id INTEGER NOT NULL REFERENCES wrestlers(id) ON DELETE CASCADE,
  team_number INTEGER DEFAULT 1,
  entrance_order INTEGER,
  UNIQUE(match_id, wrestler_id)
);