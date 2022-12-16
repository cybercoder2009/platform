use std::collections::BTreeSet; 
use rocket::serde::{Serialize, Deserialize};

use crate::constants::Role;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub password: String,
    pub token: String,
    pub role: Role,

    pub id_groups: BTreeSet<String>,
}

#[derive(Deserialize)]
pub struct UserPost {
    pub id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserPatchPassword {
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserPatchRole {
    pub role: Role,
}