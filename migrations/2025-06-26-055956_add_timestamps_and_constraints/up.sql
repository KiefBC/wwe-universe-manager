-- Add timestamps to all tables (SQLite doesn't support CURRENT_TIMESTAMP in ALTER TABLE)
-- So we'll add nullable columns first, then update them, then make them NOT NULL
ALTER TABLE users ADD COLUMN created_at DATETIME;
ALTER TABLE users ADD COLUMN updated_at DATETIME;

ALTER TABLE wrestlers ADD COLUMN created_at DATETIME;
ALTER TABLE wrestlers ADD COLUMN updated_at DATETIME;

ALTER TABLE titles ADD COLUMN created_at DATETIME;
ALTER TABLE titles ADD COLUMN updated_at DATETIME;

ALTER TABLE shows ADD COLUMN created_at DATETIME;
ALTER TABLE shows ADD COLUMN updated_at DATETIME;

-- Set initial values for all existing records
UPDATE users SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;
UPDATE wrestlers SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;
UPDATE titles SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;
UPDATE shows SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add constraints
-- Unique username constraint
CREATE UNIQUE INDEX idx_users_username ON users (username);

-- Check constraints for wrestler stats (SQLite doesn't support CHECK constraints in ALTER TABLE)
-- We'll handle these in the application layer for now

-- Add triggers to update updated_at timestamps
CREATE TRIGGER update_users_updated_at 
    AFTER UPDATE ON users
    FOR EACH ROW
    BEGIN
        UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER update_wrestlers_updated_at 
    AFTER UPDATE ON wrestlers
    FOR EACH ROW
    BEGIN
        UPDATE wrestlers SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER update_titles_updated_at 
    AFTER UPDATE ON titles
    FOR EACH ROW
    BEGIN
        UPDATE titles SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER update_shows_updated_at 
    AFTER UPDATE ON shows
    FOR EACH ROW
    BEGIN
        UPDATE shows SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;