-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE company (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);

CREATE TABLE candidate (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES company(id),
    lastname VARCHAR NOT NULL,
    firstname VARCHAR NOT NULL,
    file_name VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    motivation TEXT NOT NULL
);

INSERT INTO company (name)
VALUES ('DPS');

INSERT INTO company (name)
VALUES ('SyneidoLAB');

INSERT INTO company (name)
VALUES ('Hobbynote');

INSERT INTO company (name)
VALUES ('Elvis');

INSERT INTO company (name)
VALUES ('Pictural health');

INSERT INTO company (name)
VALUES ('SAKARA');

INSERT INTO company (name)
VALUES ('Les poupées russes');

INSERT INTO company (name)
VALUES ('Logic-Design');
