-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR,
    role VARCHAR NOT NULL
);

CREATE TABLE login_history
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);