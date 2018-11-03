#![allow(proc_macro_derive_resolution_fallback)]

use diesel::Insertable;
use diesel::Queryable;
use schema::users;
use uuid::Uuid;

// Diesel database representation.
#[derive(Debug, Clone, Queryable, PartialEq)]
pub struct User {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

// Struct for creating users
#[derive(Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}