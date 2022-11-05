use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct Ticket {
  pub email: String,
  pub description: String
}

