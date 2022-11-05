
mod structs;

#[macro_use] extern crate rocket;

use std::string;

use rocket::State;
use rocket::tokio::sync::{Mutex};
use rocket::serde::json::{Json, Value, json};
use crate::structs::ticket::*;

type TicketList = Mutex<Vec<Ticket>>;
type Tickets<'r> = &'r State<TicketList>;

fn getSeverity(text: &str) -> String {
    if text.contains("kill") {
        return "High".to_owned()
    } else if text.contains("die") {
        return "moderate".to_owned()
    } else {
        return "low".to_owned()
    }
}

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
    let mut new_ticket = ticket.0.clone();
    new_ticket.severity = Some(getSeverity(&new_ticket.description));
    tickets.push(new_ticket.clone());
    print!("{:?}", new_ticket.severity);
    rocket::serde::json::Json(new_ticket)
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])
        .manage(TicketList::new(vec![]))
}
