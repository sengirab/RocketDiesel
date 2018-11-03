#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate uuid;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::Outcome;
use rocket::Request;
use rocket::request;
use rocket::request::FromRequest;
use rocket::State;
use std::ops::Deref;

pub mod schema;
pub mod models;
pub use models::user::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    Pool::new(manager).expect("db pool")
}

pub struct Database(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Database {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Database, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Database(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Database {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}