use mongodb::bson::{self, doc, Document};
use rocket::futures::StreamExt;
use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::constants::{Code, Role, COLL_LOGS, MAX_LIMIT};
use crate::service_db::build_regex;
use crate::struct_auth::Auth;
use crate::struct_response::Response;
use crate::struct_server::Server;

/// action: http-get <br>
/// url: /api/logs/q/{keyword}/l/{level}/f/{from}/t/{to}/s/{sort}/so/{sort_order}/s/{skip}/l/{limit} <br><br>
#[get("/logs/q/<keyword>/l/<level>/f/<from>/t/<to>/s/<sort>/so/<sort_order>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>,
    auth: Auth,
    keyword: &str,
    level: &str,
    from: u32,
    to: u32,
    sort: &str,
    mut sort_order: i32,
    skip: u32,
    mut limit: u32,
) -> Json<Response<Value>> {
    limit = std::cmp::min(limit, MAX_LIMIT);
    if sort_order != 1 {sort_order = -1;}

    info!(
        "logs get keyword={} level={} from={} to={} sort={} sort_order={} skip={} limit={}",
        keyword, level, from, to, sort, sort_order, skip, limit
    );
    let coll = server.db.collection::<Document>(COLL_LOGS);
    let mut pipelines: Vec<Document> = vec![];
    let mut query_match_and: Vec<Document> = vec![];
    query_match_and.push(doc! {"timestamp": {"$gte": from}});
    query_match_and.push(doc! {"timestamp": {"$lte": to}});
    if auth.role != Role::Admin as isize {
        query_match_and.push(doc! {"initiator": auth.id});
    }
    if level.trim() != "" {
        query_match_and.push(doc! {"level": level});
    }
    if keyword.trim() != "" {
        query_match_and.push(doc! {"keyword": {"$regex": build_regex(keyword)}});
    }
    pipelines.push(doc!{"$match": {"$and": query_match_and}});
    pipelines.push(doc!{"$project": {"_id": {"$toString": "$_id"}, "initiator": 1, "level": 1, "keyword": 1, "timestamp": 1, "extra": 1}});
    pipelines.push(doc!{"$sort": {sort: sort_order}});
    pipelines.push(doc!{"$facet": {"records": [{"$skip": skip}, {"$limit": limit}], "total": [{"$count": "count"}]}});

    let res = coll
        .aggregate(pipelines, None)
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    let mut data: Vec<Value> = vec![];
    let mut total: i32 = 0;
    let total_array = res.get_array("total").unwrap().to_owned();
    if total_array.len() == 1 {
        total = total_array[0]
            .as_document()
            .unwrap()
            .get("count")
            .unwrap()
            .as_i32()
            .unwrap();
    }
    let records = res.get_array("records").unwrap();
    for record in records {
        data.push(bson::from_bson(record.clone()).unwrap());
    }

    Json(Response {
        code: Code::Success,
        total,
        data,
    })
}

#[cfg(test)]
mod test {}