#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate database;
extern crate rocket;
extern crate rocket_contrib;

use database::User;
pub use routes::Routable;

pub mod routes;

pub mod requests;

fn format_api(path: &str) -> String {
    String::from("/api") + path
}

pub fn init_rocket() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount(&format_api(User::PATH), User::ROUTES()).launch();
}
