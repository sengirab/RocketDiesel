use database::NewUser;

pub struct UserRegisterRequest {
    first_name: String,
    last_name: String,
    username: String,
    password: String
}

impl From<UserRegisterRequest> for NewUser {
    fn from(request: UserRegisterRequest) -> NewUser {
        NewUser {
            first_name: request.first_name,
            last_name: request.last_name,
            username: request.username,
            password: request.password
//            hash_password(&request.password)
//            .expect("Password could not be hashed."),
        }
    }
}