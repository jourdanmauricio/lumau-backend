use crate::schema::*;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

// TODO implement From<> and Into<> traits for all of the three.
// TODO make a macro that automatically creates/guesses such structs.

/// Represents a User object from the database.
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::admin_users)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub url: String,
    pub front_deploy: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub dni: Option<String>,
    pub status: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = admin_users)]
pub struct RegisterUser {
    pub name: String,
    pub url: String,
    pub front_deploy: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub dni: Option<String>,
    pub status: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Represents a JSON data form filled request that will turn to a `User`.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub url: String,
    pub front_deploy: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub dni: String,
    pub status: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub name: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub url: String,
    pub password: String,
}
