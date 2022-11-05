-- many to one relation to ticket

create table note (
  id SERIAL PRIMARY KEY NOT NULL,
  ticket_id INT REFERENCES ticket(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  author VARCHAR(255)
)