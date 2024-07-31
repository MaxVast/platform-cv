-- Your SQL goes here
ALTER TABLE users
ADD COLUMN company_id UUID;

ALTER TABLE users
ADD CONSTRAINT fk_company
FOREIGN KEY (company_id)
REFERENCES company (id);