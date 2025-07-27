-- Add detailed fields to wrestlers table
ALTER TABLE wrestlers ADD COLUMN real_name TEXT;
ALTER TABLE wrestlers ADD COLUMN nickname TEXT;
ALTER TABLE wrestlers ADD COLUMN height TEXT;
ALTER TABLE wrestlers ADD COLUMN weight TEXT;
ALTER TABLE wrestlers ADD COLUMN debut_year INTEGER;
ALTER TABLE wrestlers ADD COLUMN promotion TEXT DEFAULT 'WWE';

-- Add power ratings (1-10 scale)
ALTER TABLE wrestlers ADD COLUMN strength INTEGER DEFAULT 5 CHECK(strength >= 1 AND strength <= 10);
ALTER TABLE wrestlers ADD COLUMN speed INTEGER DEFAULT 5 CHECK(speed >= 1 AND speed <= 10);
ALTER TABLE wrestlers ADD COLUMN agility INTEGER DEFAULT 5 CHECK(agility >= 1 AND agility <= 10);
ALTER TABLE wrestlers ADD COLUMN stamina INTEGER DEFAULT 5 CHECK(stamina >= 1 AND stamina <= 10);
ALTER TABLE wrestlers ADD COLUMN charisma INTEGER DEFAULT 5 CHECK(charisma >= 1 AND charisma <= 10);
ALTER TABLE wrestlers ADD COLUMN technique INTEGER DEFAULT 5 CHECK(technique >= 1 AND technique <= 10);

-- Add content fields
ALTER TABLE wrestlers ADD COLUMN biography TEXT;
ALTER TABLE wrestlers ADD COLUMN trivia TEXT;

-- Create signature moves table with color coding for primary vs secondary finishers
CREATE TABLE signature_moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wrestler_id INTEGER NOT NULL,
    move_name TEXT NOT NULL,
    move_type TEXT NOT NULL CHECK(move_type IN ('primary', 'secondary')),
    created_at DATETIME,
    updated_at DATETIME,
    FOREIGN KEY (wrestler_id) REFERENCES wrestlers (id) ON DELETE CASCADE
);

-- Create index for faster lookups
CREATE INDEX idx_signature_moves_wrestler_id ON signature_moves(wrestler_id);

-- Add constraint to limit moves per wrestler (2 primary + 2 secondary = 4 total)
-- Note: SQLite doesn't support CHECK constraints on multiple rows directly,
-- so this will be enforced in the application logic