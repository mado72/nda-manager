-- Migration to add name field to users table
-- This adds a name column for storing user's full name or display name

-- Add the name column (allowing NULL initially for existing users)
ALTER TABLE users ADD COLUMN name TEXT;

-- For existing users, set a default name based on username
-- In a real scenario, you might want to prompt users to update their names
UPDATE users SET name = username WHERE name IS NULL;

-- Make the name column NOT NULL after setting default values
-- We need to recreate the table since SQLite doesn't support modifying constraints directly
CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    stellar_public_key TEXT UNIQUE NOT NULL,
    stellar_secret_key TEXT NOT NULL,
    roles TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

-- Copy data to new table
INSERT INTO users_new (id, username, name, stellar_public_key, stellar_secret_key, roles, created_at)
SELECT id, username, name, stellar_public_key, stellar_secret_key, roles, created_at
FROM users;

-- Drop old table and rename new one
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- Recreate indexes
CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_stellar_public_key ON users (stellar_public_key);