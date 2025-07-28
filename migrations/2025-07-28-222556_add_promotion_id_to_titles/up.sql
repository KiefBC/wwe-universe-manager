-- Add promotion_id column first without foreign key constraint
ALTER TABLE titles ADD COLUMN promotion_id INTEGER NOT NULL DEFAULT 1;