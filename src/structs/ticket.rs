use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Ticket {
  pub email: String,
  pub description: String,
  pub severity: Option<String>
}

