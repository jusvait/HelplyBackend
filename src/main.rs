#[macro_use] extern crate rocket;

use rocket::serde::json::{Json, Value, json};
use rocket::form::{Form, Strict};

mod structs;

use crate::structs::ticket::*;

#[get("/")]
fn index() -> Value {
    json!({"status": "ok"})
}

#[get("/")]
fn get_many() -> Json<Ticket> {
    Json (Ticket {
        email: String::from("hansen@hansen.hansenzone"),
        description: String::from("hansen time")
    })
}

#[post("/", data = "<ticket_form>")]
async fn new(ticket_form: Form<Strict<Ticket>>) -> Value {
    json!({ "status": ticket_form.description })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])

}
