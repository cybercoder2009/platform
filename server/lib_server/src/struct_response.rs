use rocket::serde::{Serialize, Deserialize};

use crate::constants::Code;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {

    pub code: Code,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
}
