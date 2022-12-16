use std::collections::BTreeSet;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub id_bases: BTreeSet<String>,
    pub id_templates: BTreeSet<String>,
    pub id_associates: BTreeSet<String>,
    pub id_items: BTreeSet<String>,
    pub id_labels: BTreeSet<String>,
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