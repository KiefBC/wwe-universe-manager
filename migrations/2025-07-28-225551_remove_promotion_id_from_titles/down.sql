-- Add promotion_id column back to titles table
-- Default to 1 for existing records (the default promotion)
ALTER TABLE titles ADD COLUMN promotion_id INTEGER NOT NULL DEFAULT 1;
