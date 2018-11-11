extern crate rocket;

use rocket::{
    http::Status,
    request::Request,
    response::{
        Responder,
        Response,
    },
};

pub type BackendResult<T> = Result<T, BackendError>;

#[derive(Debug, Clone, PartialEq)]
pub enum BackendError {
    DatabaseError(Option<String>),
    NotFound(String),
    NotAuthorized(String),
    DatabaseUnavailable,
    InternalServerError,
    BadRequest,
    ThreadImmutable,
    IllegalToken,
    ExpiredToken,
    MissingToken,
    MalformedToken,
}

impl<'r> Responder<'r> for BackendError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();

        use BackendError::*;
        match self {
            DatabaseUnavailable => {
                build.merge("Database Could Not be Reached".to_string().respond_to(req)?);
                build.status(Status::InternalServerError).ok()
            }
            DatabaseError(db_error) => {
                if let Some(error_message) = db_error {
                    build.merge(error_message.respond_to(req)?);
                } else {
                    build.merge("Database Error".to_string().respond_to(req)?);
                }
                build.status(Status::InternalServerError).ok()
            }
            InternalServerError => build.status(Status::InternalServerError).ok(),
            NotFound(type_name) => {
                let err_message = format!("Could not find requested {}", type_name);
                Response::build_from(err_message.respond_to(req)?)
                    .status(Status::NotFound)
                    .ok()
            }
            NotAuthorized(reason) => build.merge(reason.respond_to(req)?).status(Status::Forbidden).ok(),
            BadRequest => build
                .merge("Malformed request".respond_to(req)?)
                .status(Status::BadRequest)
                .ok(),
            ThreadImmutable => build
                .merge("Thread being operated upon is locked and therefore cant be changed".respond_to(req)?)
                .status(Status::MethodNotAllowed)
                .ok(),
            IllegalToken => build
                .merge("Login token's contents do not match its signature.".respond_to(req)?)
                .status(Status::Unauthorized)
                .ok(),
            ExpiredToken => build
                .merge("Login token has expired.".respond_to(req)?)
                .status(Status::Unauthorized)
                .ok(),
            MissingToken => build
                .merge("Login token not supplied.".respond_to(req)?)
                .status(Status::Unauthorized)
                .ok(),
            MalformedToken => build
                .merge("Login token was not specified correctly.".respond_to(req)?)
                .status(Status::Unauthorized)
                .ok(),
        }
    }
}
