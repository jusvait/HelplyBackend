#[macro_use] extern crate rocket;

use rocket::serde::{json::Json};

mod structs;

use crate::structs::ticket::*;

#[get("/")]
fn index() -> Json<Ticket> {
    Json (Ticket {
        email: String::from("hansen@hansen.hansenzone"),
        description: String::from("hansen time")
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}