-- Rollback Migration 2: Drop wrestlers system

-- Drop triggers first
DROP TRIGGER IF EXISTS update_signature_moves_updated_at;
DROP TRIGGER IF EXISTS update_wrestlers_updated_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_signature_moves_wrestler_id;

-- Drop tables (signature_moves first due to foreign key dependency)
DROP TABLE IF EXISTS signature_moves;
DROP TABLE IF EXISTS wrestlers;