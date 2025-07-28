-- Drop title_holders table and indexes
DROP INDEX IF EXISTS idx_title_holders_current;
DROP INDEX IF EXISTS idx_title_holders_wrestler_id;
DROP INDEX IF EXISTS idx_title_holders_title_id;
DROP TABLE title_holders;