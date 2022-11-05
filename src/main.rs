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

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, all_options])
        .mount("/ticket", routes![get_many, get_one, create, update, create_note])
}