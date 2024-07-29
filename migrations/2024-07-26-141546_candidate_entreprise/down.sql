-- This file should undo anything in `up.sql`
DROP TABLE entreprise;
DROP TABLE candidate;

ALTER TABLE users
DROP CONSTRAINT IF EXISTS fk_entreprise;

ALTER TABLE users
DROP COLUMN IF EXISTS entreprise_id;
