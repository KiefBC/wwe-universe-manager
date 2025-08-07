-- Add indexes to optimize show roster queries
-- These indexes improve performance for exclusive wrestler assignment queries

-- Composite index for wrestler_id + is_active lookups (used in LEFT JOIN for unassigned wrestlers)
CREATE INDEX IF NOT EXISTS idx_show_rosters_wrestler_active 
ON show_rosters (wrestler_id, is_active);

-- Composite index for show_id + is_active lookups (used in get_wrestlers_for_show)
CREATE INDEX IF NOT EXISTS idx_show_rosters_show_active 
ON show_rosters (show_id, is_active);

-- Index on wrestler_id for general foreign key lookups
CREATE INDEX IF NOT EXISTS idx_show_rosters_wrestler_id 
ON show_rosters (wrestler_id);

-- Index on show_id for general foreign key lookups
CREATE INDEX IF NOT EXISTS idx_show_rosters_show_id 
ON show_rosters (show_id);