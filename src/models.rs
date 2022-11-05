use diesel::prelude::*;
use rocket::{serde::{Serialize, Deserialize}};
use chrono::{NaiveDateTime};

#[derive(Queryable)]
pub struct Ticket {
  pub id: i32,
  pub email: String,
  pub description: String,
  pub created_at: Option<NaiveDateTime>,
  pub assigned_to: Option<String>,
  pub status: String,
  pub reporter: Option<String>,
  pub reporter_email: Option<String>,
  pub severity: String,
  pub reporter_estimate: i32, 
}