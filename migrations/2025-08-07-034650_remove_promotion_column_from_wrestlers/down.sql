-- This file should undo anything in `up.sql`
-- Re-add promotion column if migration needs to be reverted
ALTER TABLE wrestlers ADD COLUMN promotion TEXT;
