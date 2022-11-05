use diesel::prelude::*;
use rocket::{serde::{Serialize, Deserialize}};
use chrono::{NaiveDateTime};
use crate::schema::ticket;

#[derive(Queryable, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
  pub id: i32,
  pub email: String,
  pub description: String,
  pub created_at: NaiveDateTime,
  pub assigned_to: Option<String>,
  pub status: String,
  pub reporter: Option<String>,
  pub reporter_email: Option<String>,
  pub severity: String,
  pub reporter_estimate: i32, 
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = ticket)]
pub struct NewTicket {
  pub email: String,
  pub description: String,
  pub assigned_to: Option<String>,
  pub status: String,
  pub reporter: Option<String>,
  pub reporter_email: Option<String>,
  pub severity: Option<String>,
  pub reporter_estimate: i32
}

#[derive(Insertable, Serialize, Deserialize, Clone, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = ticket)]
pub struct UpdateTicket {
  pub assigned_to: Option<String>,
  pub status: String,
}