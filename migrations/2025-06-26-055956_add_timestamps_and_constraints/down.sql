-- Remove triggers
DROP TRIGGER IF EXISTS update_users_updated_at;
DROP TRIGGER IF EXISTS update_wrestlers_updated_at;
DROP TRIGGER IF EXISTS update_titles_updated_at;
DROP TRIGGER IF EXISTS update_shows_updated_at;

-- Remove unique index
DROP INDEX IF EXISTS idx_users_username;

-- Remove timestamp columns
-- Note: SQLite doesn't support dropping columns directly, so we would need to recreate tables
-- For now, we'll just comment this out as it's destructive
-- ALTER TABLE users DROP COLUMN created_at;
-- ALTER TABLE users DROP COLUMN updated_at;
-- ALTER TABLE wrestlers DROP COLUMN created_at;
-- ALTER TABLE wrestlers DROP COLUMN updated_at;
-- ALTER TABLE titles DROP COLUMN created_at;
-- ALTER TABLE titles DROP COLUMN updated_at;
-- ALTER TABLE shows DROP COLUMN created_at;
-- ALTER TABLE shows DROP COLUMN updated_at;