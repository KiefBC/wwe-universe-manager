-- Consolidated Migration 2: Create wrestlers system
-- This migration creates the wrestlers table with all enhanced features and signature moves

-- Create wrestlers table with all columns (base + enhanced details + power ratings)
CREATE TABLE wrestlers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Base wrestler information
    name TEXT NOT NULL,
    gender TEXT NOT NULL,
    wins INTEGER NOT NULL DEFAULT 0,
    losses INTEGER NOT NULL DEFAULT 0,
    -- Enhanced details
    real_name TEXT,
    nickname TEXT,
    height TEXT,
    weight TEXT,
    debut_year INTEGER,
    -- Power ratings (1-10 scale)
    strength INTEGER DEFAULT 5 CHECK(strength >= 1 AND strength <= 10),
    speed INTEGER DEFAULT 5 CHECK(speed >= 1 AND speed <= 10),
    agility INTEGER DEFAULT 5 CHECK(agility >= 1 AND agility <= 10),
    stamina INTEGER DEFAULT 5 CHECK(stamina >= 1 AND stamina <= 10),
    charisma INTEGER DEFAULT 5 CHECK(charisma >= 1 AND charisma <= 10),
    technique INTEGER DEFAULT 5 CHECK(technique >= 1 AND technique <= 10),
    -- Content fields
    biography TEXT,
    -- System fields
    is_user_created BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create signature moves table with color coding for primary vs secondary finishers
CREATE TABLE signature_moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wrestler_id INTEGER NOT NULL,
    move_name TEXT NOT NULL,
    move_type TEXT NOT NULL CHECK(move_type IN ('primary', 'secondary')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wrestler_id) REFERENCES wrestlers (id) ON DELETE CASCADE
);

-- Add indexes for performance
CREATE INDEX idx_signature_moves_wrestler_id ON signature_moves(wrestler_id);

-- Add triggers to automatically update updated_at timestamps
CREATE TRIGGER update_wrestlers_updated_at 
    AFTER UPDATE ON wrestlers
    FOR EACH ROW
    BEGIN
        UPDATE wrestlers SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER update_signature_moves_updated_at 
    AFTER UPDATE ON signature_moves
    FOR EACH ROW
    BEGIN
        UPDATE signature_moves SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;