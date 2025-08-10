-- Rollback Migration 3: Drop shows

-- Drop trigger first
DROP TRIGGER IF EXISTS update_shows_updated_at;

-- Drop table
DROP TABLE IF EXISTS shows;