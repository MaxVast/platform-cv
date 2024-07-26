-- Your SQL goes here
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
