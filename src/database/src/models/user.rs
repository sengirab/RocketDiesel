#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::schema::users;

// Diesel database representation.
#[derive(Debug, Clone, CRD, Queryable, PartialEq)]
#[model = "NewUser"]
#[table_name = "users"]
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

impl User {
    pub fn test_create(insert: NewUser, conn: &PgConnection) {
        diesel::insert_into(users::table)
            .values(&insert)
            .execute(conn).unwrap();
    }
}


