-- Rollback Migration 4: Drop match booking system

-- Drop trigger first
DROP TRIGGER IF EXISTS update_matches_updated_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_match_participants_team;
DROP INDEX IF EXISTS idx_match_participants_wrestler_id;
DROP INDEX IF EXISTS idx_match_participants_match_id;
DROP INDEX IF EXISTS idx_matches_scheduled_date;
DROP INDEX IF EXISTS idx_matches_title_id;
DROP INDEX IF EXISTS idx_matches_winner_id;
DROP INDEX IF EXISTS idx_matches_show_id;

-- Drop tables in reverse order (respecting foreign key dependencies)
DROP TABLE IF EXISTS match_participants;
DROP TABLE IF EXISTS matches;