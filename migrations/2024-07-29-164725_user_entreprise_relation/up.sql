-- Your SQL goes here
ALTER TABLE users
ADD COLUMN entreprise_id UUID;

ALTER TABLE users
ADD CONSTRAINT fk_entreprise
FOREIGN KEY (entreprise_id)
REFERENCES entreprise (id);