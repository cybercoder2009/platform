use std::cmp::min;
use std::collections::BTreeSet;
use rocket::serde::json::Json;
use rocket::tokio::spawn;
use rocket::State;
use serde_json::Value;
use rumqttc::AsyncClient;
use lib_canvas::template::Template;
use lib_vendors::yala::Message;

use crate::constants::*;
use crate::service_data::{populate, render_label_image};
use crate::struct_auth::Auth;
use crate::struct_user::User;
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_db::*;
use crate::struct_group::Group;
use crate::struct_label::Label;

#[post("/<id_group>/items", format = "application/json", data = "<items>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, items: Json<Vec<Value>>,
) -> Json<Response<String>> {
    
    // filter
    if items.len() > MAX_POST { return error(ERR_MAX_LIMIT); } 
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group= opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(
        auth.role == Role::Admin ||
        user.id_groups.contains(id_group) || 
        group.id_associates.contains(&auth.id)
    ){ return error(ERR_ACCESS_DENIED); }

    let mut data: Vec<String> = vec![];
    let mut keys_items: Vec<String> = vec![];
    let mut keys_items_labels: Vec<String> = vec![];
    let mut values_items_labels: Vec<BTreeSet<String>> = vec![];

    // prepare data
    let mut data_matched: Vec<(Label, (Value, Template))> = vec![];
    for item in items.iter() {

        // filter
        if item.get("id").is_none()       || !item["id"].is_string()
        || item.get("keyword").is_none()  || !item["keyword"].is_string()
        || item.get("template").is_none() || !item["template"].is_string() {continue;}
        let id_item: String = item["id"].as_str().unwrap().to_string();
        let _key_item_labels: String = key_item_labels(id_group, &id_item);
        let opt_id_labels: Option<BTreeSet<String>> = server.db.read::<BTreeSet<String>>(&_key_item_labels);
        if opt_id_labels.is_none() { /*info!("!!!! 0");*/ continue;}
        let id_labels: BTreeSet<String> = opt_id_labels.unwrap();
        if id_labels.is_empty() { /*info!("!!!! 1");*/ continue;}

        group.id_items.insert(id_item.clone(), item["keyword"].to_string().trim().to_lowercase());
        data.push(id_item.clone());
        keys_items.push(key_item(id_group, &id_item));
        keys_items_labels.push(key_item_labels(id_group, &id_item));
        values_items_labels.push(BTreeSet::new());

        for id_label in id_labels {
            let opt_label: Option<Label> = server.db.read::<Label>(&key_label(id_group, &id_label));
            if opt_label.is_none() { /*info!("!!!! 2");*/ continue; }
            let label: Label = opt_label.unwrap();
            let opt_template: Option<Template> = server.db.read::<Template>(&key_template(
                id_group,
                &id_template(item["template"].as_str().unwrap(), label.width, label.height)
            ));
            if opt_template.is_none() { /*info!("!!!! 3");*/ continue; }
            // info!("!!! data_matched label={} item={}", label.id, id_item);
            data_matched.push((label, (item.clone(), opt_template.unwrap())));
        }
    }

    // populate & dispatch
    info!("!!! data_matched.len()={}", data_matched.len());
    for (label, (item, template)) in data_matched.into_iter() {

        let mut template_0: Template = template.clone();
        let mqtt_0: AsyncClient = server.mqtt.clone();
        let images_0: String = server.images.clone();
        spawn(async move {

            populate(item.as_object().unwrap(), &mut template_0);

            let message: Message = render_label_image(&label, &template_0, &images_0);
            mqtt_0.publish(
                "test/refresh/queue",
                rumqttc::QoS::AtLeastOnce,
                false,
                serde_json::to_string(&message).unwrap(),
            ).await.unwrap();
        });
    }

    // update g-$id_group
    server.db.write::<Group>(&_key_group, &group);

    // update g-$id_group-i-$id_item
    server.db.write_batch::<Value>(&keys_items, &items);

    // update g-$id_group-i-$id_item-labels
    server.db.write_batch_if_not_exist::<BTreeSet<String>>(&keys_items_labels, &values_items_labels);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(data),
    })
}


#[get("/<id_group>/items/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<Value>> {

    // fitlers
    let opt_group: Option<Group> = server.db.read::<Group>(&key_group(id_group));
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(
        auth.role == Role::Admin ||
        user.id_groups.contains(id_group) || 
        group.id_associates.contains(&auth.id)
    ){ return error(ERR_ACCESS_DENIED); }

    limit = std::cmp::min(limit, MAX_LIMIT);
    info!("items get id_group={} keyword={} skip={} limit={}", id_group, keyword, skip, limit);
    let keyword_0: String = keyword.trim().to_lowercase();
    
    group.id_items.retain(
        |id, _keyword|
        id.contains(keyword) || _keyword.contains(&keyword_0)
    );
    let total: usize = group.id_items.len();
    let keys: Vec<String> = group.id_items.keys()
        .cloned().collect::<Vec<String>>()[skip .. min(skip + limit, total)]
        .iter().map(|id|{key_item(id_group, id)}).collect();
    let items: Vec<Value> = server.db.read_batch::<Value>(&keys);

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(items),
    })
}

#[delete("/<id_group>/items/<id_item>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, id_item: &'r str,
) -> Json<Response<&'r str>> {

    // fitlers
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    if !group.id_items.contains_key(id_item) { return error("item-not-found"); }
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(
        auth.role == Role::Admin ||
        user.id_groups.contains(id_group) || 
        group.id_associates.contains(&auth.id)
    ){ return error(ERR_ACCESS_DENIED); }

    // update g-$id_group
    group.id_items.remove(id_item);
    server.db.write::<Group>(&_key_group, &group);

    // update g-{id_group}-t-{id_item}
    server.db.delete(&key_item(id_group, id_item));

    // update g-{id_group}-t-{id_item}-labels
    server.db.delete(&key_item_labels(id_group, id_item));

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[cfg(test)]
mod route_items {

    use lib_utilities::random;
    use rocket::serde::json::{json, Json, Value};
    use rocket::State;

    use crate::constants::Code;
    use crate::route_groups::post as group_post;
    use crate::route_items::{delete, get, post};
    use crate::struct_auth::Auth;
    use crate::struct_group::GroupPost;
    use crate::struct_server::Server;

    #[async_test]
    async fn test() {
        
        // env
        let server: Server = Server::_mock();

        // groups post
        let res = group_post(
            State::from(&server), Auth::_mock_user(),
            Json(GroupPost {
                name: random::string(10),
            }),
        ).await.into_inner();
        let id_group: String = res.data.unwrap()[0].clone();

        // items get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_items.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());

        // items post
        let id_item: &str = "abc001";
        let item: Value = json!({
            "id": id_item,
            "keyword": "apple juice",
            "template": "regular",
        });

        let items: Vec<Value> = vec![item];
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(items),
        ).await.into_inner();
        println!("[route_items.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.data.unwrap()[0], id_item);

        // items get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_items.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // items delete
        let res = delete(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            &id_item,
        ).await.into_inner();
        println!("[route_bases.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        // items get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_items.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());
    }
}
