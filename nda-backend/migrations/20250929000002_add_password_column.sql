-- Add password column to users table
-- Migration: 20250929000002_add_password_column.sql
-- Description: Adds hashed password column for user authentication

ALTER TABLE users ADD COLUMN password_hash TEXT NOT NULL DEFAULT '';

-- Update existing users with a temporary hash (will be updated when users set passwords)
UPDATE users SET password_hash = 'temp_hash_needs_reset' WHERE password_hash = '';