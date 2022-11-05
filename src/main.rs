
mod structs;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::tokio::sync::{Mutex};
use rocket::serde::json::{Json, Value, json};
use crate::structs::ticket::*;

type TicketList = Mutex<Vec<Ticket>>;
type Tickets<'r> = &'r State<TicketList>;

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
    tickets.push(ticket.0.clone());
    ticket
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])
        .manage(TicketList::new(vec![]))
}
