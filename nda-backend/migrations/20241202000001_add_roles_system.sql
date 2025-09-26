-- Migration to add roles system to users table
-- This migration adds a roles column and migrates existing user_type data

-- Add the new roles column
ALTER TABLE users ADD COLUMN roles TEXT;

-- Migrate existing user_type data to roles format
-- Convert "client" -> ["client"] and "supplier" -> ["supplier"]
UPDATE users SET roles = '["' || user_type || '"]';

-- Make roles column NOT NULL after migration
-- (SQLite doesn't support adding NOT NULL columns directly with data)
CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    stellar_public_key TEXT UNIQUE NOT NULL,
    stellar_secret_key TEXT NOT NULL,
    roles TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

-- Copy data to new table
INSERT INTO users_new (id, username, stellar_public_key, stellar_secret_key, roles, created_at)
SELECT id, username, stellar_public_key, stellar_secret_key, roles, created_at
FROM users;

-- Drop old table and rename new one
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- Recreate indexes
CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_stellar_public_key ON users (stellar_public_key);

-- Update processes foreign key (if needed)
-- The foreign key relationship should be maintained automatically