
mod structs;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::tokio::sync::{Mutex};
use rocket::serde::json::{Json, Value, json};
use crate::structs::ticket::*;

/*
type TicketList = Mutex<Vec<Ticket>>;
type Tickets<'r> = &'r State<TicketList>;
*/
use self::models::*;
use diesel::prelude::*;
use helply_backend::*;

fn main() {
    use self::schema::ticket::dsl::*;

    let connection = &mut establish_connection();
    let results = ticket
        .limit(5)
        .load::<Ticket>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.description);
    }
}
/* 
#[get("/")]
fn index() -> Value {
    json!({"status": "ok"})
}

#[get("/")]
async fn get_many(list: Tickets<'_>) -> Json<Vec<Ticket>> {
    let tickets = list.lock().await;
    Json (tickets.to_vec())
}

#[post("/", format = "json", data = "<ticket>")]
async fn new(ticket: Json<Ticket>, list: Tickets<'_>) -> Json<Ticket> {
    let mut tickets = list.lock().await;
    let new_ticket = Ticket {
        created_at: Some(chrono::Utc::now()), 
        ..ticket.0.clone()
    };
    tickets.push(new_ticket.clone());
    return Json( new_ticket );
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])
        .manage(TicketList::new(vec![]))
}
*/