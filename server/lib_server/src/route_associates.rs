use std::cmp::min;
use rocket::serde::json::Json;
use rocket::State;
use lib_utilities::validate::email;

use crate::constants::*;
use crate::struct_db::{key_user, key_group};
use crate::struct_group::Group;
use crate::struct_auth::Auth;
use crate::struct_user::User;
use crate::struct_response::Response;
use crate::struct_server::Server;

#[post("/<id_group>/associates", format = "application/json", data = "<associates>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, associates: Json<Vec<String>>,
) -> Json<Response<String>> {
    
    // filters
    if associates.len() > MAX_POST { return error(ERR_MAX_LIMIT); } 
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(auth.role == Role::Admin || user.id_groups.contains(id_group))
    { return error(ERR_ACCESS_DENIED); }

    let mut data: Vec<String> = vec![];
    for associate in associates.iter() {
        if email(&associate) {
            let id_associate: String = associate.trim().to_string();
            group.id_associates.insert(id_associate.clone());
            data.push(id_associate);
        }
    }
    server.db.write::<Group>(&_key_group, &group);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(data),
    })
}

#[get("/<id_group>/associates/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<String>> {

    // fitlers
    let opt_group: Option<Group> = server.db.read::<Group>(&key_group(id_group));
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let group: Group = opt_group.unwrap();
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(auth.role == Role::Admin || user.id_groups.contains(id_group))
    { return error(ERR_ACCESS_DENIED); }

    limit = min(limit, MAX_LIMIT);
    info!("associates get id_group={} keyword={} skip={} limit={}", id_group, keyword, skip, limit);
    let total: usize = group.id_associates.len();
    let associates: Vec<String> = group.id_associates
        .into_iter().collect::<Vec<String>>()[skip .. min(skip + limit, total)].to_vec();

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(associates),
    })
}

#[delete("/<id_group>/associates/<id_associate>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id_group: &'r str, id_associate: &'r str,
) -> Json<Response<String>> {

    // fitlers
    let _key_group: String = key_group(id_group);
    let opt_group: Option<Group> = server.db.read::<Group>(&_key_group);
    if opt_group.is_none() { return error(ERR_GROUP_NOT_FOUND); }
    let mut group: Group = opt_group.unwrap();
    if !group.id_associates.contains(id_associate) { return error("associate-not-found"); }
    let user: User = server.db.read::<User>(&key_user(&auth.id)).unwrap();
    if !(auth.role == Role::Admin || user.id_groups.contains(id_group))
    { return error(ERR_ACCESS_DENIED); }

    // update g-$id_group
    group.id_associates.remove(id_associate);
    server.db.write(&_key_group, &group);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: None,
    })
}

#[cfg(test)]
mod route_associates {

    use lib_utilities::random;
    use rocket::serde::json::Json;
    use rocket::State;

    use crate::constants::Code;
    use crate::route_associates::{delete, get, post};
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

        // associates post
        let id_associate: String = random::string(10);
        let associates: Vec<String> = vec![format!("{} ", id_associate)];
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(associates.clone()),
        ).await.into_inner();
        println!("[route_associates.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(id_associate, res.data.unwrap()[0]);

        // associates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_associates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // associates post with same associate id
        let res = post(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            Json(associates),
        ).await.into_inner();
        println!("[route_associates.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        
        // associates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_associates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(1, res.total.unwrap());

        // associates delete
        let res= delete(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            &id_associate,
        ).await.into_inner();
        println!("[route_associates.delete] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        // associates get
        let res = get(
            State::from(&server), Auth::_mock_user(),
            &id_group,
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_associates.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(0, res.total.unwrap());
    }
}
