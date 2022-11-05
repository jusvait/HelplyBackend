// https://diesel.rs/guides/getting-started
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env};
use models::{Ticket, NewTicket, UpdateTicket, Note, NewNote};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_one_ticket(ticket_id: i32) -> (Ticket, Vec<Note>) {
    let connection = &mut establish_connection();
    let t = self::schema::ticket::dsl::ticket
        .filter(self::schema::ticket::dsl::id.eq(ticket_id))
        .first::<Ticket>(connection)
        .expect("Could not find ticket");
    let n = self::schema::note::dsl::note
        .filter(self::schema::note::dsl::ticket_id.eq(ticket_id))
        .load::<Note>(connection)
        .expect("Could not find notes");

    return (t,n)
}

pub fn get_all_tickets() -> Vec<Ticket> {
    use self::schema::ticket::dsl::*;
    let connection = &mut establish_connection();
    return ticket
        .load::<Ticket>(connection)
        .expect("Error loading posts");
}

pub fn create_ticket(ticket: NewTicket) -> Ticket {
    use crate::schema::ticket;
    let connection = &mut establish_connection();

    let new_ticket = NewTicket {
        severity: Some(get_severity(&ticket.description)),
        ..ticket
    };

    diesel::insert_into(ticket::table)
        .values(&new_ticket)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn update_ticket(ticket_id: i32, new_ticket: UpdateTicket) -> Ticket {
    use self::schema::ticket::dsl::{ticket, id};
    let connection = &mut establish_connection();

    diesel::update(ticket.find(id))
        .set(new_ticket)
        .execute(connection)
        .expect("Error creating new Ticket");

    return get_one_ticket(ticket_id).0;
}

pub fn add_note(ticket_id: i32, new_note: NewNote) -> Note {
    use crate::schema::note;
    let connection = &mut establish_connection();

    let new_note = NewNote {
        ticket_id: Some(ticket_id),
        ..new_note
    };

    diesel::insert_into(note::table)
        .values(&new_note)
        .get_result(connection)
        .expect("Error creating new Note")
}

fn get_severity(text: &str, estimate: &i32 ) -> String {
    let weight = 0;
    weight += estimate + 1;

    if text.contains("kill") {
        weight += 4
    } else if text.contains("die") {
        weight += 2
    }

    if weight >= 10 {
        return "high".to_owned()
    } else if weight >= 5 {
        return "moderate".to_owned()
    } else {
        return "low".to_owned();
    }
}