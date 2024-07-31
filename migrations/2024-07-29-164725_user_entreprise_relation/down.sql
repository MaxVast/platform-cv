-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP CONSTRAINT IF EXISTS fk_company;

ALTER TABLE users
DROP COLUMN IF EXISTS company_id;