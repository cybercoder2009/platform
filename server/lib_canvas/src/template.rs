#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {

    pub id: String,

    pub keyword: String,

    pub width: u32,

    pub height: u32,

    pub thumbnail: String,

    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {

    pub kind: String,                

    pub x: i32,

    pub y: i32,

    pub rotation: i32,

    #[serde(skip_serializing_if = "Option::is_none")] // circles, stars
    pub margin: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // rect, barcode
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // rect
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // image
    pub naturalWidth: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")] // image
    pub naturalHeight: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // circle, circles
    pub radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // star, stars
    pub innerRadius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // star, stars
    pub outerRadius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // rect, circle, circles, star, starts
    pub strokeWidth: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // circles, stars, qrcode
    pub fill: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // circles, stars, qrcode
    pub fill_light: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // rect, circle, circles, star, stars
    pub stroke: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // star, stars
    pub numPoints: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // circles, stars
    pub value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // circles, stars
    pub max: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // text, barcode
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // text
    pub fontSize: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // text
    pub fontStyle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // text
    pub fontFamily: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // image
    pub base64: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // image
    pub base64_raw: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // text, barcode, qrcode, circles, stars
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] // barcode
    pub format: Option<String>,
}

impl Template {
    pub fn _mock() -> Template {
        Template {
            id: format!("-{}x{}", 212, 103),
            keyword: "".to_string(),
            thumbnail: "".to_string(),
            width: 212,
            height: 103,
            elements: vec![],
        }
    }
}
