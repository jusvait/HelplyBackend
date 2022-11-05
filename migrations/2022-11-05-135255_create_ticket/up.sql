-- Your SQL goes here
CREATE TABLE ticket (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    assigned_to VARCHAR(255),
    status VARCHAR(255) NOT NULL,
    reporter VARCHAR(255),
    reporter_email VARCHAR(255),
    severity VARCHAR(255) NOT NULL,
    reporter_estimate INT NOT NULL
);