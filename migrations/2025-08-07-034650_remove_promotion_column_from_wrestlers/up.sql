-- Remove deprecated promotion column from wrestlers table
-- Per CLAUDE.md architecture: wrestlers are global entities assigned to shows via show_rosters table
ALTER TABLE wrestlers DROP COLUMN promotion;
