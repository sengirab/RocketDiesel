use rocket::response::Failure;
use rocket::response::status;
use rocket::Route;
use rocket_contrib::Json;

use database::Database;
use database::NewUser;
use database::User;
use routes::Routable;

#[get("/get_users", format = "application/json")]
fn get_users(_connection: Database) -> Result<status::Accepted<Json<String>>, Failure> {
    Ok(
        status::Accepted(Some(Json(String::from("Hello world"))))
    )
}

#[get("/create_user", format = "application/json")]
fn create_user(connection: Database) -> Result<status::Accepted<Json<String>>, Failure> {
    let user = NewUser {
        first_name: String::from("Benjamin"),
        last_name: String::from("Bloot"),
        username: String::from("Sengira"),
        password: String::from("12345"),
    };
    User::test_create(user, &connection);

    Ok(
        status::Accepted(Some(Json(String::from("Created"))))
    )
}

// Export the ROUTES and their path
impl Routable for User {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            get_users,
            create_user
        ]
    };
    const PATH: &'static str = "/user/";
}