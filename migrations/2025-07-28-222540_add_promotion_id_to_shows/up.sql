-- Add promotion_id column first without foreign key constraint
ALTER TABLE shows ADD COLUMN promotion_id INTEGER NOT NULL DEFAULT 1;