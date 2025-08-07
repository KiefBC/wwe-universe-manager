-- Rollback Migration 4: Drop titles system

-- Drop triggers first
DROP TRIGGER IF EXISTS update_title_holders_updated_at;
DROP TRIGGER IF EXISTS update_titles_updated_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_show_rosters_show_id;
DROP INDEX IF EXISTS idx_show_rosters_wrestler_id;
DROP INDEX IF EXISTS idx_show_rosters_show_active;
DROP INDEX IF EXISTS idx_show_rosters_wrestler_active;
DROP INDEX IF EXISTS idx_title_holders_current;
DROP INDEX IF EXISTS idx_title_holders_wrestler_id;
DROP INDEX IF EXISTS idx_title_holders_title_id;

-- Drop tables in reverse order of creation (respecting foreign key dependencies)
DROP TABLE IF EXISTS show_rosters;
DROP TABLE IF EXISTS title_holders;
DROP TABLE IF EXISTS titles;