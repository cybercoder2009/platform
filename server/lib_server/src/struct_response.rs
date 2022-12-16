use rocket::serde::Serialize;

use crate::constants::Code;

#[derive(Serialize, Debug)]
pub struct Response<'a, T> {

    pub code: Code<'a>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
}
