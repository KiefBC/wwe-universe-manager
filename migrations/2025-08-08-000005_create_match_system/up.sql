-- Consolidated Migration 4: Create match booking system
-- This migration creates the complete match system with participants and all constraints

-- Create matches table with comprehensive match details
CREATE TABLE matches (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    show_id INTEGER NOT NULL,
    match_name TEXT,
    match_type TEXT NOT NULL,
    match_stipulation TEXT,
    scheduled_date DATE,
    match_order INTEGER,
    winner_id INTEGER,
    is_title_match BOOLEAN NOT NULL DEFAULT FALSE,
    title_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (show_id) REFERENCES shows (id) ON DELETE CASCADE,
    FOREIGN KEY (winner_id) REFERENCES wrestlers (id) ON DELETE SET NULL,
    FOREIGN KEY (title_id) REFERENCES titles (id) ON DELETE SET NULL
);

-- Create match_participants table for wrestler participation in matches
CREATE TABLE match_participants (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    match_id INTEGER NOT NULL,
    wrestler_id INTEGER NOT NULL,
    team_number INTEGER,
    entrance_order INTEGER,
    FOREIGN KEY (match_id) REFERENCES matches (id) ON DELETE CASCADE,
    FOREIGN KEY (wrestler_id) REFERENCES wrestlers (id) ON DELETE CASCADE
);

-- Add performance indexes for match queries
CREATE INDEX idx_matches_show_id ON matches (show_id);
CREATE INDEX idx_matches_winner_id ON matches (winner_id);
CREATE INDEX idx_matches_title_id ON matches (title_id);
CREATE INDEX idx_matches_scheduled_date ON matches (scheduled_date);

-- Add performance indexes for match participants
CREATE INDEX idx_match_participants_match_id ON match_participants (match_id);
CREATE INDEX idx_match_participants_wrestler_id ON match_participants (wrestler_id);
CREATE INDEX idx_match_participants_team ON match_participants (match_id, team_number);

-- Add trigger to automatically update updated_at timestamp for matches
CREATE TRIGGER update_matches_updated_at 
    AFTER UPDATE ON matches
    FOR EACH ROW
    BEGIN
        UPDATE matches SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;