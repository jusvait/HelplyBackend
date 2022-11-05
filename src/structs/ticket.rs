use rocket::{serde::{Serialize, Deserialize}};
use chrono::{DateTime,  Utc};
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
#[derive(Queryable)]
pub struct Ticketasd {
  pub id: u128,
  pub email: String,
  pub description: String,
  pub created_at: Option<DateTime<Utc>>,
  pub assigned_to: Option<String>,
  pub status: String,
  pub reporter: Option<String>,
  pub reporter_email: Option<String>,
  pub severity: String,
  pub reporter_estimate: u128, 
}
