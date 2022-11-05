// https://diesel.rs/guides/getting-started
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewTicket, Ticket};

pub fn create_ticket(ticket: NewTicket) -> Ticket {
    use crate::schema::ticket;
    let connection = &mut establish_connection();

    let new_ticket = NewTicket { 
        severity: get_severity(&ticket.description),
        ..ticket 
    };

    diesel::insert_into(ticket::table)
        .values(&new_ticket)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn get_all_tickets() -> Vec<Ticket> {
    use self::schema::ticket::dsl::*;
    let connection = &mut establish_connection();
    return ticket
        .load::<Ticket>(connection)
        .expect("Error loading posts");
}

fn get_severity(text: &str) -> String {
    if text.contains("kill") {
        return "High".to_owned()
    } else if text.contains("die") {
        return "moderate".to_owned()
    } else {
        return "low".to_owned()
    }
}