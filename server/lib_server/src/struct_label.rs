use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Label {
    pub id: String,
    pub mac: String,
    pub version: String,
    pub id_item: String,
    pub width: u32,
    pub height: u32,
}