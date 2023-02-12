use std::collections::{BTreeSet, BTreeMap};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    // pub id_bases: BTreeSet<String>,
    pub id_associates: BTreeSet<String>,
    pub id_templates: BTreeMap<String, String>, // $id_template ------> $keyword(lowercase)
    pub id_items: BTreeMap<String, String>,     // $id_item     ------> $keyword(lowercase)
    pub id_labels: BTreeMap<String, String>,    // $id_label    ------> $keyword(lowercase)
}

#[derive(Deserialize)]
pub struct GroupPost {
    pub name: String,
}

#[derive(Deserialize)]
pub struct GroupPut {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct GroupGet {
    pub id: String,
    pub name: String,
    pub items: usize,
    pub labels: usize,
    pub templates: usize,
    pub associates: usize,
    // pub bases: usize,
}

#[derive(Serialize, Debug)]
pub struct GroupsSummary {
    pub groups: usize,
    pub items: usize,
    pub labels: usize,
    pub templates: usize,
    pub associates: usize,
    // pub bases: usize,
}