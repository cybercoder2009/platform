
use std::cmp::min;
use std::collections::{BTreeSet, BTreeMap};
use rocket::serde::json::Json;
use rocket::State;
use lib_utilities::random::string;

use crate::constants::{Code, Role, MAX_LIMIT, error, ERR_GROUP_NOT_FOUND};
use crate::struct_db::*;
use crate::struct_auth::Auth;
use crate::struct_user::User;
use crate::struct_group::*;
use crate::struct_response::Response;
use crate::struct_server::Server;

#[post("/", format = "application/json", data = "<group>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    group: Json<GroupPost>,
) -> Json<Response<String>> {

    let id_group: String = string(16);
    
    // update g-all
    let mut groups: BTreeSet<String> = server.db.read::<BTreeSet<String>>(KEY_GROUPS).unwrap();
    groups.insert(id_group.clone());
    server.db.write::<BTreeSet<String>>(KEY_GROUPS, &groups); 
    
    // update g-$id_group    
    let _key_group: String = key_group(&id_group);
    server.db.write::<Group>(
        &_key_group,
        &Group {
            id: id_group.clone(),
            name: group.0.name,
            // id_bases: BTreeSet::new(),
            id_associates: BTreeSet::new(),
            id_templates: BTreeMap::new(),
            id_items: BTreeMap::new(),
            id_labels: BTreeMap::new(),
        }
    );

    // update u-$id_user
    let _key_user: String = key_user(&auth.id);
    let mut user: User = server.db.read::<User>(&_key_user).unwrap();
    user.id_groups.insert(id_group.clone());
    server.db.write::<User>(&_key_user, &user);

    Json(Response{
        code: Code::Success(()),
        total: None,
        data: Some(vec![id_group])
    })
}

#[patch("/<id>", format = "application/json", data = "<group>")]
pub async fn patch<'r>(
    server: &'r State<Server>, auth: Auth,
    id: &'r str, group: Json<GroupPut>,
) -> Json<Response<&'r str>> {

    // filters
    let _key_user: String = key_user(&auth.id);
    let user: User = server.db.read::<User>(&_key_user).unwrap();
    if !user.id_groups.contains(&id.to_string()) { return error("access-denied");  }
    let _key_group: String = key_group(id);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error("group-not-found"); }
    
    // update g-$id_group
    let mut _group: Group = opt_group.unwrap();
    _group.name = group.0.name;
    server.db.write::<Group>(&_key_group, &_group);

    Json(Response{
        code: Code::Success(()),
        total: None,
        data: Some(vec![id])
    })
}

#[get("/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<GroupGet>> {

    // auth
    let id_groups: BTreeSet<String>;
    match auth.role {
        Role::Admin => {
            id_groups = server.db.read::<BTreeSet<String>>(KEY_GROUPS).unwrap();
        },
        Role::User => {
            let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
            id_groups = user.id_groups;
        }
    }

    // filter
    let total: usize = id_groups.len();
    if skip > total { return Json(Response {code: Code::Success(()), total: Some(total), data: Some(vec![])}); }
    
    limit = min(limit, MAX_LIMIT);
    info!("groups get keyword={} skip={} limit={}", keyword, skip, limit);
    let keys: Vec<String> = id_groups
        .iter().collect::<Vec<&String>>()[skip .. min(skip + limit, total)]
        .iter().map(|id|{key_group(&id)}).collect();
    let groups: Vec<Group> = server.db.read_batch::<Group>(&keys);
    let mut data: Vec<GroupGet> = vec![];
    for group in groups {
        data.push(GroupGet {
            id: group.id,
            name: group.name,
            items: group.id_items.len(),
            labels: group.id_labels.len(),
            templates: group.id_templates.len(),
            associates: group.id_associates.len(),
        });
    }

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(data),
    })
}

#[delete("/<id>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id: &'r str,
) -> Json<Response<&'r str>> {

    // filters
    let _key_group: String = key_group(id);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }

    // update u-$id_user
    let _key_user: String = key_user(&auth.id);
    let mut user: User = server.db.read::<User>(&_key_user).unwrap();
    user.id_groups.remove(id);
    server.db.write::<User>(&_key_user, &user);

    // update g-$id_group
    server.db.delete(&_key_group);
    
    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[get("/summary")]
pub async fn summary<'r>(
    server: &'r State<Server>, auth: Auth,
) -> Json<Response<GroupsSummary>> {

    // auth
    let id_groups: BTreeSet<String>;
    match auth.role {
        Role::Admin => {
            id_groups = server.db.read::<BTreeSet<String>>(KEY_GROUPS).unwrap();
        },
        Role::User => {
            let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
            id_groups = user.id_groups;
        }
    }

    info!("groups summary");
    let keys: Vec<String> = id_groups
        .iter().map(|id|{key_group(&id)}).collect();
    let groups: Vec<Group> = server.db.read_batch::<Group>(&keys);
    let mut data: GroupsSummary = GroupsSummary {
        groups: id_groups.len(),
        items: 0,
        labels: 0,
        templates: 0,
        associates: 0,
    };
    for group in groups {
        data.items += group.id_items.len();
        data.labels += group.id_labels.len();
        data.templates += group.id_templates.len();
        data.associates += group.id_associates.len();
    }

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(vec![data]),
    })
}

#[cfg(test)]
mod route_groups {

    use lib_utilities::random;
    use rocket::serde::json::Json;
    use rocket::State;

    use crate::constants::Code;
    use crate::route_groups::*;
    use crate::struct_auth::Auth;
    use crate::struct_group::{GroupPut, GroupPost};
    use crate::struct_server::Server;

    #[async_test]
    async fn test() {

        // env
        let server: Server = Server::_mock();

        /* group post */
        let res = post(
            State::from(&server), Auth::_mock_user(),
            Json(GroupPost {
                name: random::string(10),
            }),
        ).await.into_inner();
        println!("[route_groups.post] {:?}\r\n", res);
        let id_group: String = res.data.unwrap()[0].clone();
        assert_eq!(Code::Success(()), res.code);

        /* group patch */
        let name0: String = random::string(10);
        let res = patch(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(GroupPut {
                name: name0.clone(),
            }),
        ).await.into_inner();
        println!("[route_groups.patch] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(id_group, res.data.unwrap()[0]);

        /* groups get count before delete */
        let res = get(
            State::from(&server), Auth::_mock_user(),
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_groups.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(name0, res.data.unwrap()[0].name);
        let groups_count_before_delete: usize = res.total.unwrap();
    
        /* group delete and receive group id */
        let res = delete(
            State::from(&server), Auth::_mock_user(),
            &id_group
        ).await.into_inner();
        println!("[route_groups.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        /* get groups count after delete */
        let res = get(
            State::from(&server), Auth::_mock_user(),
            "",
            0,
            10,
        ).await.into_inner();
        println!("[route_groups.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.total.unwrap() + 1, groups_count_before_delete);

        /* summary groups */
        let res = summary(
            State::from(&server), Auth::_mock_user(),
        ).await.into_inner();
        println!("[route_groups.summary] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.data.unwrap().len(), 1);
    }
}
