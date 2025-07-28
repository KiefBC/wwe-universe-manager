-- Add new fields to titles table for enhanced functionality
ALTER TABLE titles ADD COLUMN title_type TEXT NOT NULL DEFAULT 'Singles';
ALTER TABLE titles ADD COLUMN division TEXT NOT NULL DEFAULT 'World';
ALTER TABLE titles ADD COLUMN prestige_tier INTEGER NOT NULL DEFAULT 1;
ALTER TABLE titles ADD COLUMN gender TEXT NOT NULL DEFAULT 'Mixed';
ALTER TABLE titles ADD COLUMN show_id INTEGER;
ALTER TABLE titles ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;

-- Add foreign key constraint for show_id
-- Note: SQLite doesn't support adding foreign key constraints to existing tables
-- But we'll document the intended relationship
-- FOREIGN KEY (show_id) REFERENCES shows (id)