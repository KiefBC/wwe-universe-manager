-- Remove promotion_id column from titles table
-- Titles should be global, not promotion-specific
ALTER TABLE titles DROP COLUMN promotion_id;
