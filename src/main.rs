#[macro_use] extern crate rocket;

use rocket::State;
use rocket::tokio::sync::{Mutex};
use rocket::serde::json::{Json, Value, json};

type TicketList = Mutex<Vec<Ticket>>;
type Tickets<'r> = &'r State<TicketList>;

use self::models::*;
use diesel::prelude::*;
use helply_backend::*;

fn get_severity(text: &str) -> String {
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
    use self::schema::ticket::dsl::*;
    let connection = &mut establish_connection();
    let results = ticket
        .load::<Ticket>(connection)
        .expect("Error loading posts");
    Json (results)
}

#[post("/", format = "json", data = "<ticket>")]
async fn new(ticket: Json<NewTicket>) -> Value {
    let connection = &mut establish_connection();
    let ticket = create_post(connection, ticket.0);
    return json!({"w": "hello!"});
}

#[launch]
fn stage() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ticket", routes![get_many, new])
        .manage(TicketList::new(vec![]))
}