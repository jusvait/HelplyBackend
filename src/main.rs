#[macro_use] extern crate rocket;

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

#[put("/<ticket_id>", format = "json", data = "<ticket>")]
async fn update(ticket_id: i32, ticket: Json<UpdateTicket>) -> () {
    update_ticket(ticket_id, ticket.0);
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new, update])
}