use std::cmp::min;
use std::sync::Arc;
use std::collections::BTreeSet;
use rocket::serde::json::Json;
use rocket::tokio::spawn;
use rocket::State;
use serde_json::Value;
use rumqttc::AsyncClient;
use lib_canvas::template::Template;

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
    if !user.id_groups.contains(id_group) && !group.id_associates.contains(&auth.id) { 
        return error(ERR_ACCESS_DENIED); }

    let mut data: Vec<String> = vec![];
    let mut keys_items: Vec<String> = vec![];
    let mut keys_items_labels: Vec<String> = vec![];
    let mut values_items_labels: Vec<BTreeSet<String>> = vec![];
    for item in items.iter() {

        if item.get("id").is_none()       || !item["id"].is_string()
        || item.get("keyword").is_none()  || !item["keyword"].is_string()
        || item.get("template").is_none() || !item["template"].is_string() {continue;}
   
        // filter
        let id_item: String = item["id"].as_str().unwrap().to_string();
        if id_item.trim().is_empty() { continue; }

        group.id_items.insert(id_item.clone(), item["keyword"].to_string().trim().to_lowercase());
        keys_items.push(key_item(id_group, &id_item));
        keys_items_labels.push(key_item_labels(id_group, &id_item));
        values_items_labels.push(BTreeSet::new());
        data.push(id_item.clone());

        let _key_item_labels: String = key_item_labels(id_group, &id_item);
        let mqtt_0: AsyncClient = server.mqtt.clone();
        let images_0: String = server.images.clone();
        let db_0: Arc<DB> = server.db.clone();
        let item_0: Value = item.clone();
        let id_group_0: String = id_group.to_string();

        // populate & dispatch
        spawn(async move {

            let opt_id_labels: Option<BTreeSet<String>> = db_0.read::<BTreeSet<String>>(&_key_item_labels);
            if opt_id_labels.is_none() { return (); }
            let id_labels: BTreeSet<String> = opt_id_labels.unwrap();
            if id_labels.is_empty() { return (); }

            let template_keyword: &str = item_0["template"].as_str().unwrap();
            let mut keys_labels: Vec<String> = vec![];
            id_labels.iter().for_each(|id_label: &String|{
                keys_labels.push(key_label(&id_group_0, id_label));
            });
            let mut labels: Vec<Label> = db_0.read_batch::<Label>(&keys_labels);
            let mut keys_templates: Vec<String> = vec![];
            labels.iter().for_each(|label: &Label|{
                keys_templates.push(key_template(
                    &id_group_0,
                    &id_template(&template_keyword, label.width, label.height)
                ));
            });
            
            // templates.len() <= labels.len()
            let mut templates: Vec<Template> = db_0.read_batch::<Template>(&keys_templates);
            templates.iter_mut().for_each(|template: &mut Template|{
                
                populate(item_0.as_object().unwrap(), template);

                labels.retain(|label: &Label|{

                    // dispatch
                    let mqtt_1: AsyncClient = mqtt_0.clone();
                    let label_0: Label = label.clone();
                    let template_0: Template = template.clone();
                    let images_1: String = images_0.clone();
                    spawn(async move {
                        
                        let message = render_label_image(&label_0, &template_0, &images_1);
                        mqtt_1.publish(
                            "test/refresh/queue",
                            rumqttc::QoS::AtLeastOnce,
                            false,
                            serde_json::to_string(&message).unwrap(),
                        ).await.unwrap();
                    });
                    true
                });
            });
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
    if !user.id_groups.contains(id_group) && !group.id_associates.contains(&auth.id) { 
        return error(ERR_ACCESS_DENIED); }

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
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) && !group.id_associates.contains(&auth.id) { 
        return error(ERR_ACCESS_DENIED); }
    if !group.id_items.contains_key(id_item) { return error("item-not-found"); }

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
