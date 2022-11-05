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

#[get("/<ticket_id>")]
async fn get_one(ticket_id: i32) -> Json<(Ticket, Vec<Note>)> {
    let result = get_one_ticket(ticket_id);
    Json (result)
}

#[post("/", format = "json", data = "<ticket>")]
async fn create(ticket: Json<NewTicket>) -> Json<Ticket> {
    let ticket = create_ticket(ticket.0);
    Json (ticket)
}

#[patch("/<ticket_id>", format = "json", data = "<ticket>")]
async fn update(ticket_id: i32, ticket: Json<UpdateTicket>) -> Json<Ticket> {
    let ticket = update_ticket(ticket_id, ticket.0);
    Json (ticket)
}

#[post("/<ticket_id>/note", format = "json", data = "<note>")]
async fn create_note(ticket_id: i32, note: Json<NewNote>) -> Json<Note> {
    let note = add_note(ticket_id, note.0);
    Json (note)
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, get_one, create, update, create_note])
}