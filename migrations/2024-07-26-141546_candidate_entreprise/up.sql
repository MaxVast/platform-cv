-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE entreprise (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);

CREATE TABLE candidate (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entreprise_id UUID NOT NULL REFERENCES entreprise(id),
    lastname VARCHAR NOT NULL,
    firstname VARCHAR NOT NULL,
    file_name VARCHAR NOT NULL,
    motivation TEXT NOT NULL
);

ALTER TABLE users
ADD COLUMN entreprise_id UUID;

ALTER TABLE users
ADD CONSTRAINT fk_entreprise
FOREIGN KEY (entreprise_id)
REFERENCES entreprise (id);
