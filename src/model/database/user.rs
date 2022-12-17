#![allow(dead_code)]

use chrono::{DateTime, Local};
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    password: String,
    create_at: DateTime<Local>,
    update_at: DateTime<Local>,
    status: String,
    role: String,
}
