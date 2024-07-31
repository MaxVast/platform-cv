-- Your SQL goes here
CREATE TABLE job_offers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company_id UUID NOT NULL REFERENCES company(id),
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    requirements VARCHAR ,
    location VARCHAR NOT NULL,
    remote VARCHAR,
    employment_type VARCHAR NOT NULL,
    salary FLOAT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE
);
