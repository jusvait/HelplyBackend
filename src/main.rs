#[macro_use] extern crate rocket;

use rocket::tokio::sync::{Mutex};
use rocket::serde::json::{Json, Value, json};

use self::models::*;
use helply_backend::*;

#[get("/")]
fn index() -> Value {
    json!({"status": "ok"})
}

#[get("/")]
async fn get_many() -> Json<Vec<Ticket>> {
    let results = get_all_tickets();
    Json (results)
}

#[post("/", format = "json", data = "<ticket>")]
async fn new(ticket: Json<NewTicket>) -> Json<Ticket> {
    let ticket = create_ticket(ticket.0);
    Json (ticket)
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])
}