-- Create title_holders junction table for championship history and current holders
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

-- Create indexes for performance
CREATE INDEX idx_title_holders_title_id ON title_holders (title_id);
CREATE INDEX idx_title_holders_wrestler_id ON title_holders (wrestler_id);
CREATE INDEX idx_title_holders_current ON title_holders (title_id, held_until) WHERE held_until IS NULL;