-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP CONSTRAINT IF EXISTS fk_entreprise;

ALTER TABLE users
DROP COLUMN IF EXISTS entreprise_id;