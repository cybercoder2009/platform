use rocket::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

use crate::struct_response::Response;

pub const CONFIG_PATH: &str = "config.toml";

pub const ID_ADMIN: &str = "admin@reducing.ca";
pub const ID_USER: &str  = "user@reducing.ca";
pub const ID_USER0: &str  = "user0@reducing.ca";

pub const PW: &str = "password";

pub const TOKEN_ADMIN: &str = "ogK9PdfcL0PLIrWi";
pub const TOKEN_USER: &str  = "P0MgnlTGMRGCirMW";
pub const TOKEN_USER0: &str  = "P0MgnlTGMRGCirMX";

pub const ID_GROUP_ADMIN: &str = "yREaGRL9HJnoY1Vb";
pub const ID_GROUP_USER: &str  = "t6zNqWHbjg2C21uY";

pub const MAX_LIMIT: usize = 300; // limit for http get
pub const MAX_POST: usize = 50;

pub const ERR_ACCESS_DENIED: &str = "access-denied";
pub const ERR_GROUP_NOT_FOUND: &str = "group-not-found";
pub const ERR_MAX_LIMIT: &str = "max-limit";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Copy)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, PartialEq, Debug, Responder)]
pub enum Code<'a> {
    Success(()),
    Error(&'a str),
}

pub fn error<'a, T>(msg: &'a str) -> Json<Response<'a, T>> {
    Json(Response {
        code: Code::Error(msg),
        total: None,
        data: None
    })
}