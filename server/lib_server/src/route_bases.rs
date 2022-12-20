use std::cmp::min;
use rocket::serde::json::Json;
use rocket::State;

use crate::constants::*;
use crate::struct_db::{key_user, key_group};
use crate::struct_auth::Auth;
use crate::struct_user::User;
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_group::Group;

#[post("/<id_group>/bases", format = "application/json", data = "<bases>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, bases: Json<Vec<String>>,
) -> Json<Response<String>> {

    // filters
    if bases.len() > MAX_POST { return error(ERR_MAX_LIMIT); } 
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }

    let mut group: Group = opt_group.unwrap();
    let mut data: Vec<String> = vec![];
    for base in bases.iter() {
        let id_base: String = base.trim().to_string();
        group.id_bases.insert(id_base.clone());
        data.push(id_base);
    }

    // update g-$id_group
    server.db.write::<Group>(&_key_group, &group);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(data),
    })
}

#[get("/<id_group>/bases/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<String>> {

    // fitlers
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) { return error(ERR_ACCESS_DENIED); }
    let opt_group: Option<Group> = server.db.read::<Group>(&key_group(id_group));
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }

    limit = min(limit, MAX_LIMIT);
    info!("bases get id_group={} keyword={} skip={} limit={}", id_group, keyword, skip, limit);
    let group: Group = opt_group.unwrap();
    let total: usize = group.id_bases.len();
    let bases: Vec<String> = group.id_bases
        .into_iter().collect::<Vec<String>>()[skip .. min(skip + limit, total)].to_vec();

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(bases),
    })
}

#[delete("/<id_group>/bases/<id_base>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, id_base: &'r str,
) -> Json<Response<&'r str>> {
    
    // fitlers
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !user.id_groups.contains(id_group) { return error(ERR_ACCESS_DENIED); }
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    if !group.id_bases.contains(id_base) { return error("base-not-found"); }

    // update g-$id_group
    group.id_bases.remove(id_base);
    server.db.write(&_key_group, &group);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[cfg(test)]
mod route_bases {

    use lib_utilities::random;
    use rocket::serde::json::Json;
    use rocket::State;

    use crate::constants::Code;
    use crate::route_bases::{delete, get, post};
    use crate::route_groups::post as group_post;
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
        println!("[route_groups.post] {:?}\r\n", res);
        let id_group: String = res.data.unwrap()[0].clone();

        // bases post
        let id_base: String = random::string(10);
        let bases: Vec<String> = vec![format!("{} ", id_base)];
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(bases.clone()),
        ).await.into_inner();
        println!("[route_bases.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(id_base, res.data.unwrap()[0]);

        // bases get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_bases.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // bases post with same base id
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(bases),
        ).await.into_inner();
        println!("[route_bases.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        
        // bases get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_bases.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // bases delete
        let res= delete(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            &id_base,
        ).await.into_inner();
        println!("[route_bases.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        // bases get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_bases.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());
    }
}
