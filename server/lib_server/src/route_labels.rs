use std::cmp::min;
use std::collections::BTreeSet;
use rocket::serde::json::Json;
use rocket::tokio::spawn;
use rocket::State;
use serde_json::Value;
use lib_canvas::template::Template;
use lib_utilities::random::u16;
use lib_vendors::yala::Message;

use crate::constants::*;
use crate::struct_db::*;
use crate::struct_auth::Auth;
use crate::struct_label::Label;
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_user::User;
use crate::struct_group::Group;
use crate::service_data::{populate, render_label_image};

#[post("/<id_group>/labels", format = "application/json", data = "<labels>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, labels: Json<Vec<Label>>,
) -> Json<Response<String>> {

    // filter
    if labels.len() > MAX_POST { return error(ERR_MAX_LIMIT); } 
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group= opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) && !group.id_associates.contains(&auth.id) { 
        return error(ERR_ACCESS_DENIED); }
        
    let mut data: Vec<String> = vec![];
    let mut ks: Vec<String> = vec![];
    for label in labels.iter() {

        // filter
        let id_label: String = label.id.clone();
        if id_label.trim().is_empty() { continue; }

        ks.push(key_label(id_group, &id_label));
        group.id_labels.insert(id_label.clone(), id_label.clone().trim().to_lowercase());
        data.push(label.id.clone());

        // update g-$id_group-i-$id_item-labels
        if label.id_item != "" {
            let _key_item_labels: String = key_item_labels(&group.id, &label.id_item);
            match server.db.read::<BTreeSet<String>>(&_key_item_labels){
                Some(mut labels) => {
                    labels.insert(label.id.to_string());
                    server.db.write::<BTreeSet<String>>(&_key_item_labels, &labels);
                },
                None => {
                    let mut labels: BTreeSet<String> = BTreeSet::new();
                    labels.insert(label.id.to_string());
                    server.db.write::<BTreeSet<String>>(&_key_item_labels, &labels);
                },
            }
        }

        /* unbind */ if label.id_item == "" {
            let message: Message = Message::new(
                u16(),
                &label.id,
                &label.mac,
                &label.firmware,
                2,
                0,
                Vec::new(),
            );
            server.mqtt.publish(
                "test/refresh/queue",
                rumqttc::QoS::AtLeastOnce,
                false,
                serde_json::to_string(&message).unwrap(),
            ).await.unwrap();
            
        /* bind */ } else {

            // item
            let opt_item: Option<Value> = server.db.read::<Value>(&key_item(id_group, &label.id_item));
            if opt_item.is_none() { continue; }
            let item: Value = opt_item.unwrap();

            // template
            let _id_template: String = id_template(item["template"].as_str().unwrap(), label.width, label.height);
            let opt_template: Option<Template> = server.db.read::<Template>(&key_template(id_group, &_id_template));
            if opt_template.is_none() { continue; }
            let mut template: Template = opt_template.unwrap();
            
            let mqtt_0 = server.mqtt.clone();
            let images_0: String = server.images.clone();
            let label_0: Label = label.clone();

            // populate & dispatch
            spawn(async move {

                populate(&item.as_object().unwrap(), &mut template);

                let message = render_label_image(&label_0, &template, &images_0);
                
                mqtt_0.publish(
                    "test/refresh/queue",
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(&message).unwrap(),
                ).await.unwrap();
            });
        }
    }
    
    // update g-$id_group
    server.db.write::<Group>(&_key_group, &group);

    // update g-$id_group-l-$id_label
    server.db.write_batch::<Label>(&ks, &labels);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(data),
    })
}

#[get("/<id_group>/labels/q/<keyword>/s/<skip>/l/<limit>")]
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
    info!("labels get _id_group={} keyword={} skip={} limit={}", id_group, keyword, skip, limit); 
    let keyword_0: String = keyword.trim().to_lowercase();
    group.id_labels.retain(|_, _keyword|_keyword.contains(&keyword_0));
    let total: usize = group.id_labels.len();
    let keys: Vec<String> = group.id_labels.keys()
        .cloned().collect::<Vec<String>>()[skip .. min(skip + limit, total)]
        .iter().map(|id|{key_label(id_group, id)}).collect();
    let labels: Vec<Value> = server.db.read_batch::<Value>(&keys);

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(labels),
    })
}

#[delete("/<id_group>/labels/<id_label>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, id_label: &'r str,
) -> Json<Response<&'r str>> {

    // fitlers
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) && !group.id_associates.contains(&auth.id) { 
        return error(ERR_ACCESS_DENIED); }
    if !group.id_labels.contains_key(id_label) { return error("label-not-found"); }

    // update g-$id_group
    group.id_labels.remove(id_label);
    server.db.write::<Group>(&_key_group, &group);

    let _key_label: String = key_label(id_group, id_label);
    if let Some(label) = server.db.read::<Label>(&_key_label){
        
        // update g-$id_group-i-$id_item-labels
        if label.id_item != "" {
            let _key_item_labels: String = key_item_labels(id_group, id_label);
            let mut labels: BTreeSet<String> = server.db.read::<BTreeSet<String>>(&_key_item_labels).unwrap();
            labels.remove(id_label);
            server.db.write::<BTreeSet<String>>(&_key_item_labels, &labels);
        }

        // update g-{id_group}-l-{id_label}
        server.db.delete(&_key_label);
    }

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[cfg(test)]
mod route_labels {

    use lib_utilities::random;
    use rocket::serde::json::Json;
    use rocket::State;

    use crate::constants::Code;
    use crate::route_groups::post as group_post;
    use crate::route_labels::{delete, get, post};
    use crate::struct_auth::Auth;
    use crate::struct_group::GroupPost;
    use crate::struct_label::Label;
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
        println!("[route_groups.post] {:?}\r\n", res);
        let id_group: String = res.data.unwrap()[0].clone();

        // labels get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_labels.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());

        // labels post
        let id_label: &str = "abc001";
        let label: Label = Label {
            id: id_label.to_string(),
            mac: "mac".to_string(),
            firmware: "firmware".to_string(),
            id_item: "id_item".to_string(),
            width: 213,
            height: 102,
        };
        let labels: Vec<Label> = vec![label];
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(labels),
        ).await.into_inner();
        println!("[route_labels.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.data.unwrap()[0], id_label);

        // labels get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_labels.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // labels delete
        let res = delete(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            &id_label,
        ).await.into_inner();
        println!("[route_labels.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        // labels get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_labels.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());
    }
}
