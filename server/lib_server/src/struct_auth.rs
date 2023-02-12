use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Serialize, Deserialize};
use lib_utilities::random::string;

use crate::struct_server::{Server};
use crate::constants::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth {
    pub id: String,
    pub role: Role,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthPost {
    pub id: String,
    pub password: String,
}

#[derive(Debug)]
pub enum AuthError {
    Anonymous,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = AuthError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Auth, Self::Error> {
        let server: &Server = req.rocket().state::<Server>().unwrap();
        match req.headers().get_one("Authorization") {
            Some(token) => {
                match server.cache.lock().unwrap().get(token) {
                    Some(auth) => Outcome::Success(auth.clone()),
                    None => Outcome::Failure((Status::Forbidden, AuthError::Anonymous)),
                }
            },
            None => Outcome::Failure((Status::Forbidden, AuthError::Anonymous)),
        }
    }
}

impl Auth {
    pub fn _mock_admin() -> Self {Self {id: ID_ADMIN.to_string(), role: Role::Admin, token: string(10)}}
    pub fn _mock_user() -> Self {Self {id: ID_USER.to_string(), role: Role::User, token: string(10)}}
}

impl AuthPost {
    pub fn _mock_admin() -> AuthPost {AuthPost {id: ID_ADMIN.to_string(), password: PW.to_string()}}
    pub fn _mock_user() -> AuthPost {AuthPost {id: ID_USER.to_string(), password: PW.to_string()}}
}