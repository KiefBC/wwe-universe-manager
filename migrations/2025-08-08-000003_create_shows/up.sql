-- Migration 3: Create shows
-- This migration creates the shows table for wrestling events

-- Create shows table
CREATE TABLE shows (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Add trigger to automatically update updated_at timestamp
CREATE TRIGGER update_shows_updated_at 
    AFTER UPDATE ON shows
    FOR EACH ROW
    BEGIN
        UPDATE shows SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;