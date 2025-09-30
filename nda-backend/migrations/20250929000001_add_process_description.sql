-- Migration to add description field to processes table
-- This adds a description column for storing detailed process descriptions

-- Add the description column (allowing NULL initially for existing processes)
ALTER TABLE processes ADD COLUMN description TEXT;

-- For existing processes, set a default description based on title
-- In a real scenario, you might want to prompt users to update their descriptions
UPDATE processes SET description = 'Description for ' || title WHERE description IS NULL;

-- Make the description column NOT NULL after setting default values
-- We need to recreate the table since SQLite doesn't support modifying constraints directly
CREATE TABLE processes_new (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    encrypted_content TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL,
    FOREIGN KEY (client_id) REFERENCES users (id)
);

-- Copy data from old table to new table
INSERT INTO processes_new (id, client_id, title, description, encrypted_content, encryption_key, status, created_at)
SELECT id, client_id, title, description, encrypted_content, encryption_key, status, created_at
FROM processes;

-- Drop old table and rename new table
DROP TABLE processes;
ALTER TABLE processes_new RENAME TO processes;

-- Recreate indices
CREATE INDEX idx_processes_client_id ON processes (client_id);