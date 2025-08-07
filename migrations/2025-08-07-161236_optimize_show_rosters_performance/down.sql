-- Remove performance optimization indexes

DROP INDEX IF EXISTS idx_show_rosters_wrestler_active;
DROP INDEX IF EXISTS idx_show_rosters_show_active; 
DROP INDEX IF EXISTS idx_show_rosters_wrestler_id;
DROP INDEX IF EXISTS idx_show_rosters_show_id;