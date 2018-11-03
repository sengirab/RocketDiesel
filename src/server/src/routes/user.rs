use database::Database;
use rocket::response::status;
use rocket_contrib::Json;
use rocket::response::Failure;
use routes::Routable;
use rocket::Route;
use database::User;

#[get("/get_users", format = "application/json")]
fn get_users(_connection: Database) -> Result<status::Accepted<Json<String>>, Failure> {
    Ok(
        status::Accepted(Some(Json(String::from("Hello world"))))
    )
}

#[get("/create_user", format = "application/json")]
fn create_user(_connection: Database) -> Result<status::Accepted<Json<String>>, Failure> {
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