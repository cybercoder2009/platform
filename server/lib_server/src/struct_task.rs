use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct TaskGetSummary {
    pub date: String,
    pub tasks: i32,
    pub items: i32,
    pub labels: i32,
}
