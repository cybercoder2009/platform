use mongodb::bson::{doc, Document};
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket::State;

use lib_utilities::time::date;
use crate::constants::{Code, LogKeyword, Role, COLL_LOGS};
use crate::service_db::redis_tasks_count;
use crate::struct_auth::Auth;
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_task::TaskGetSummary;

/// action: http-get <br>
/// url: /api/tasks <br><br>
#[get("/tasks", format = "application/json")]
pub async fn get<'r>(server: &'r State<Server>, auth: Auth) -> Json<Response<usize>> {
    let data: Vec<usize> = vec![redis_tasks_count(server.redis.clone(), &auth.id).await];

    Json(Response {
        code: Code::Success,
        total: 1,
        data,
    })
}

/// action: http-get <br>
/// url: /api/tasks/summary <br><br>
#[get("/tasks/summary", format = "application/json")]
pub async fn get_summary<'r>(
    server: &'r State<Server>,
    auth: Auth,
) -> Json<Response<TaskGetSummary>> {
    let date: String = date(5);
    let mut query: Document = doc! {
        "keyword": {"$in": [
            LogKeyword::LabelsImportSuccess.as_ref(),
            LogKeyword::ItemsImportSuccess.as_ref(),
            LogKeyword::Bind.as_ref(),
        ]},
        "date": {"$gte": date}
    };
    if auth.role != Role::Admin as isize {
        query.insert("initiator", &auth.id);
    }
    let mut pipelines: Vec<Document> = vec![];
    pipelines.push(doc! {"$match": query});
    pipelines.push(doc! {"$project": {
        "_id": 0,
        "date": 1,
        "task": {"$cond": [{"$eq": ["$keyword", LogKeyword::Bind.as_ref()]}, 1, 0]},
        "item": {"$cond": [{"$eq": ["$keyword", LogKeyword::ItemsImportSuccess.as_ref()]}, 1, 0]},
        "label": {"$cond": [{"$eq": ["$keyword", LogKeyword::LabelsImportSuccess.as_ref()]}, 1, 0]},
    }});
    pipelines.push(doc! {"$group": {
        "_id": "$date",
        "tasks": {"$sum": "$task"},
        "items": {"$sum": "$item"},
        "labels": {"$sum": "$label"},
    }});
    pipelines.push(doc! {"$sort": {"_id": -1}});

    let coll = server.db.collection::<Document>(COLL_LOGS);
    let mut cursor = coll.aggregate(pipelines, None).await.unwrap();
    let mut data: Vec<TaskGetSummary> = vec![];
    while let Some(opt_res) = cursor.next().await {
        if let Ok(res) = opt_res {
            data.push(TaskGetSummary {
                date: res.get_str("_id").unwrap().to_string(),
                tasks: res.get_i32("tasks").unwrap(),
                items: res.get_i32("items").unwrap(),
                labels: res.get_i32("labels").unwrap(),
            });
        }
    }

    Json(Response {
        code: Code::Success,
        total: data.len() as i32,
        data,
    })
}

#[cfg(test)]
mod test {}
