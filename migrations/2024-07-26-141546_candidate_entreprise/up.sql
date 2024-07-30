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
    phone VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    motivation TEXT NOT NULL
);

INSERT INTO entreprise (name)
VALUES ('DPS');

INSERT INTO entreprise (name)
VALUES ('SyneidoLAB');

INSERT INTO entreprise (name)
VALUES ('Hobbynote');

INSERT INTO entreprise (name)
VALUES ('Elvis');

INSERT INTO entreprise (name)
VALUES ('Pictural health');

INSERT INTO entreprise (name)
VALUES ('SAKARA');

INSERT INTO entreprise (name)
VALUES ('Les poupées russes');

INSERT INTO entreprise (name)
VALUES ('Logic-Design');
