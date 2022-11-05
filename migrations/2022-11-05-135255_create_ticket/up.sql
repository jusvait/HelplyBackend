-- Your SQL goes here
CREATE TABLE ticket (
    id SERIAL PRIMARY KEY,
    email CHARACTER VARYING NOT NULL,
    name CHARACTER VARYING NOT NULL,
    description CHARACTER VARYING NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    assigned_to CHARACTER VARYING,
    status CHARACTER VARYING NOT NULL,
    reporter CHARACTER VARYING,
    reporter_email CHARACTER VARYING,
    severity CHARACTER VARYING NOT NULL,
    reporter_estimate INT NOT NULL
);