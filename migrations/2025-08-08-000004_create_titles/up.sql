-- Migration 4: Create titles system
-- This migration creates titles, title history, and show rosters

-- Create titles table with full feature set
CREATE TABLE titles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    current_holder_id INTEGER,
    title_type TEXT NOT NULL,
    division TEXT NOT NULL,
    prestige_tier INTEGER NOT NULL,
    gender TEXT NOT NULL,
    show_id INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_user_created BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (current_holder_id) REFERENCES wrestlers (id) ON DELETE SET NULL,
    FOREIGN KEY (show_id) REFERENCES shows (id) ON DELETE SET NULL
);

-- Create title_holders history table for championship tracking
CREATE TABLE title_holders (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title_id INTEGER NOT NULL,
    wrestler_id INTEGER NOT NULL,
    held_since TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    held_until TIMESTAMP NULL,
    event_name TEXT NULL,
    event_location TEXT NULL,
    change_method TEXT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (title_id) REFERENCES titles (id) ON DELETE CASCADE,
    FOREIGN KEY (wrestler_id) REFERENCES wrestlers (id) ON DELETE CASCADE
);

-- Create show_rosters many-to-many table for wrestler-show assignments
CREATE TABLE show_rosters (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    show_id INTEGER NOT NULL,
    wrestler_id INTEGER NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (show_id) REFERENCES shows (id) ON DELETE CASCADE,
    FOREIGN KEY (wrestler_id) REFERENCES wrestlers (id) ON DELETE CASCADE
);

-- Add performance indexes for title_holders
CREATE INDEX idx_title_holders_title_id ON title_holders (title_id);
CREATE INDEX idx_title_holders_wrestler_id ON title_holders (wrestler_id);
CREATE INDEX idx_title_holders_current ON title_holders (title_id, held_until) WHERE held_until IS NULL;

-- Add performance indexes for show_rosters (critical for exclusive assignments)
CREATE INDEX idx_show_rosters_wrestler_active ON show_rosters (wrestler_id, is_active);
CREATE INDEX idx_show_rosters_show_active ON show_rosters (show_id, is_active);
CREATE INDEX idx_show_rosters_wrestler_id ON show_rosters (wrestler_id);
CREATE INDEX idx_show_rosters_show_id ON show_rosters (show_id);

-- Add triggers to automatically update updated_at timestamps
CREATE TRIGGER update_titles_updated_at 
    AFTER UPDATE ON titles
    FOR EACH ROW
    BEGIN
        UPDATE titles SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER update_title_holders_updated_at 
    AFTER UPDATE ON title_holders
    FOR EACH ROW
    BEGIN
        UPDATE title_holders SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;