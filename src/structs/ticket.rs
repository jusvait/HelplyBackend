use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
  pub email: String,
  pub description: String
}

