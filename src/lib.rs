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

pub fn create_post(conn: &mut PgConnection, ticket: NewTicket) -> Ticket {
    use crate::schema::ticket;

    let new_post = NewTicket { ..ticket };

    diesel::insert_into(ticket::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}