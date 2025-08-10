-- Rollback Migration 1: Drop users

-- Drop trigger first
DROP TRIGGER IF EXISTS update_users_updated_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_users_username;

-- Drop table
DROP TABLE IF EXISTS users;