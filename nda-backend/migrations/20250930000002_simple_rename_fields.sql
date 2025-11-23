-- Simple migration to rename only the process tables fields
-- Rename supplier_public_key to partner_public_key in process_shares table
ALTER TABLE process_shares RENAME COLUMN supplier_public_key TO partner_public_key;

-- Rename supplier_id to partner_id in process_accesses table  
ALTER TABLE process_accesses RENAME COLUMN supplier_id TO partner_id;