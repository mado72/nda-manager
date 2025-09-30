-- Rename supplier fields to partner fields for consistency
-- This migration updates field names while preserving all data

-- Update users table: change user_type from 'supplier' to 'partner'
UPDATE users SET user_type = 'partner' WHERE user_type = 'supplier';

-- Rename supplier_public_key to partner_public_key in process_shares table
ALTER TABLE process_shares RENAME COLUMN supplier_public_key TO partner_public_key;

-- Rename supplier_id to partner_id in process_accesses table  
ALTER TABLE process_accesses RENAME COLUMN supplier_id TO partner_id;

-- Update check constraint on users table to use 'partner' instead of 'supplier'
-- Note: SQLite doesn't support ALTER TABLE MODIFY constraint, so we need to recreate the table
-- But first, let's create a temporary table with the new constraint

CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    stellar_public_key TEXT UNIQUE NOT NULL,
    stellar_secret_key TEXT NOT NULL,
    user_type TEXT NOT NULL CHECK (user_type IN ('client', 'partner')),
    created_at DATETIME NOT NULL,
    name TEXT,
    roles TEXT DEFAULT '[]',
    password TEXT
);

-- Copy all data from old table to new table
INSERT INTO users_new (id, username, stellar_public_key, stellar_secret_key, user_type, created_at, name, roles, password)
SELECT id, username, stellar_public_key, stellar_secret_key, user_type, created_at, name, roles, password
FROM users;

-- Drop old table and rename new table
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- Recreate indexes that were dropped
CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_stellar_public_key ON users (stellar_public_key);