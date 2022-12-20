use std::cmp::min;
use rocket::serde::json::Json;
use rocket::State;
use lib_canvas::template::Template;

use crate::constants::*;
use crate::struct_db::{id_template, key_user, key_group, key_template};
use crate::struct_auth::Auth;
use crate::struct_user::User;
use crate::struct_group::Group;
use crate::struct_response::Response;
use crate::struct_server::Server;

#[post("/<id_group>/templates", format = "application/json", data = "<templates>")]
pub async fn post<'r>(
    server: &'r State<Server>, _auth: Auth,
    id_group: &'r str, templates: Json<Vec<Template>>,
) -> Json<Response<String>> {

    // filters
    if templates.len() > MAX_POST { return error(ERR_MAX_LIMIT); }
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    // info!("!!! template.elements.len()={}", templates[0].elements.len());

    let mut group: Group = opt_group.unwrap();
    let mut ks: Vec<String> = vec![];
    let mut data: Vec<String> = vec![];
    for template in templates.iter() {

        let _id_template: String = id_template(template.keyword.trim(), template.width, template.height);
        ks.push(key_template(id_group, &_id_template));
        group.id_templates.insert(_id_template.clone());
        data.push(_id_template);
    }

    // update g-$id_group
    server.db.write::<Group>(&_key_group, &group);

    // update g-$id_group-t-$id_template
    info!("templates keys={:?}", &ks);
    server.db.write_batch::<Template>(&ks, &templates);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(data),
    })
}

#[get("/<id_group>/templates/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<Template>> {

    // fitlers
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) { return error(ERR_ACCESS_DENIED); }
    let opt_group: Option<Group> = server.db.read::<Group>(&key_group(id_group));
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }

    limit = min(limit, MAX_LIMIT);
    info!("templates get id_group={} keyword={} skip={} limit={}", id_group, keyword, skip, limit);
    let group: Group = opt_group.unwrap();
    let total: usize = group.id_templates.len();
    let keys: Vec<String> = group.id_templates
        .iter().collect::<Vec<&String>>()[skip .. min(skip + limit, total)]
        .iter().map(|id|{key_template(id_group, *id)}).collect();
    let templates: Vec<Template> = server.db.read_batch::<Template>(&keys);

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(templates),
    })
}

#[delete("/<id_group>/templates/<id_template>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, id_template: &'r str,
) -> Json<Response<&'r str>> {

    // fitlers
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) { return error(ERR_ACCESS_DENIED); }
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    if !group.id_templates.contains(id_template) { return error("template-not-found"); }

    // update g-$id_group
    group.id_templates.remove(id_template);
    server.db.write::<Group>(&_key_group, &group);

    // update g-$id_group-t-$id_template
    server.db.delete(&key_template(id_group, id_template));

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[cfg(test)]
mod route_templates {

    use rocket::State;
    use rocket::serde::json::Json;
    use lib_utilities::random;
    use lib_canvas::template::Template;

    use crate::constants::Code;
    use crate::route_templates::{post, get, delete};
    use crate::route_groups::post as group_post;
    use crate::struct_auth::Auth;
    use crate::struct_group::GroupPost;
    use crate::struct_server::Server;
    use crate::struct_db::id_template;

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

        // templates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_templates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());

        // templates post
        let mut template: Template = Template::_mock();
        let _id_template: String = id_template(&template.keyword, template.width, template.height);
        template.keyword = format!("{} ", template.keyword);
        let templates: Vec<Template> = vec![template];
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(templates.clone()),
        ).await.into_inner();
        println!("[route_templates.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.data.unwrap()[0], _id_template);

        // templates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_templates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // templates delete
        let res = delete(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            &_id_template,
        ).await.into_inner();
        println!("[route_templates.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        // templates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_templates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());
    }
}