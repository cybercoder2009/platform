use std::collections::BTreeSet;
use std::cmp::min;
use rocket::State;
use rocket::serde::json::Json;
use lib_utilities::hash::sha3_256;
use lib_utilities::random::string;
use lib_utilities::validate::email;

use crate::constants::{Code, Role, MAX_LIMIT, error};
use crate::struct_db::{KEY_USERS, key_user};
use crate::struct_auth::Auth;
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_user::*;

#[post("/users", format = "application/json", data = "<user>")]
pub async fn post<'r>(
    server: &'r State<Server>, auth: Auth,
    user: Json<UserPost>,
) -> Json<Response<String>> {

    // filters
    if auth.role != Role::Admin { return error("access-denied"); }
    if !email(&user.id) { return error("invalid-user-id"); }
    let _key_user: String = key_user(&user.id);
    if server.db.read::<User>(&_key_user).is_some() { return error("user-exists"); }
    
    // update u-{id_user}
    server.db.write::<User>(&_key_user, &User{
        password: sha3_256(&user.password),
        token: sha3_256(&string(32)),
        role: Role::User,
        id_groups: BTreeSet::new(),
    });

    // update u-users
    let mut id_users: BTreeSet<String> = server.db.read::<BTreeSet<String>>(KEY_USERS).unwrap_or(BTreeSet::new());
    id_users.insert(user.id.clone());
    server.db.write::<BTreeSet<String>>(KEY_USERS, &id_users);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(vec![user.id.clone()])
    })
}

#[patch("/users/<id>/password", format = "application/json", data = "<user>")]
pub async fn patch_password<'r>(
    server: &'r State<Server>, auth: Auth,
    id: &'r str, user: Json<UserPatchPassword>,
) -> Json<Response<&'r str>> {

    // filters
    if auth.role != Role::Admin && auth.id != id { return error("access-denied"); }
    let _key_user: String = key_user(id);
    let opt_user: Option<User> = server.db.read::<User>(&_key_user);
    if opt_user.is_none() { return error("user-not-found"); }

    let mut _user: User = opt_user.unwrap();
    _user.password = sha3_256(&user.password);
    server.db.write::<User>(&_key_user, &_user);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(vec![id])
    })
}

#[patch("/users/<id>/role", format = "application/json", data = "<user>")]
pub async fn patch_role<'r>(
    server: &'r State<Server>, auth: Auth,
    id: &'r str, user: Json<UserPatchRole>,
) -> Json<Response<&'r str>> {

    // filters
    if auth.role != Role::Admin { return error("access-denied"); }
    let _key_user: String = key_user(id);
    let opt_user: Option<User> = server.db.read::<User>(&_key_user);
    if opt_user.is_none() { return error("user-not-found"); }

    let mut _user: User = opt_user.unwrap();
    _user.role = user.role;
    server.db.write::<User>(&_key_user, &_user);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(vec![id])
    })
}

#[get("/users/q/<keyword>/s/<skip>/l/<limit>")]
pub async fn get<'r>(
    server: &'r State<Server>, auth: Auth,
    keyword: &'r str, skip: usize, mut limit: usize,
) -> Json<Response<String>> {
    
    // fitlers
    if auth.role != Role::Admin { return Json(Response{code: Code::Error("access-denied"), total: None, data: None}); }
    let id_users: BTreeSet<String> = server.db.read::<BTreeSet<String>>(KEY_USERS).unwrap();
    let total: usize = id_users.len();
    if skip > total { return Json(Response {code: Code::Success(()), total: Some(total), data: Some(vec![])}); }

    limit = min(limit, MAX_LIMIT);
    info!("users get keyword={} skip={} limit={}", keyword, skip, limit);
    let users: Vec<String> = id_users
        .into_iter().collect::<Vec<String>>()[skip .. min(skip + limit, total)].to_vec();

    Json(Response {
        code: Code::Success(()),
        total: Some(total),
        data: Some(users),
    })
}

#[delete("/users/<id>")]
pub async fn delete<'r>(
    server: &'r State<Server>, auth: Auth,
    id: &'r str,
) -> Json<Response<&'r str>> {
    
    // filters
    if auth.role != Role::Admin { return error("access-denied"); }

    // update u-{}
    let _key_user: String = key_user(id);
    server.db.delete(&_key_user);

    // update u-users
    let mut id_users: BTreeSet<String> = server.db.read::<BTreeSet<String>>(KEY_USERS).unwrap_or(BTreeSet::new());
    id_users.remove(id);
    server.db.write::<BTreeSet<String>>(KEY_USERS, &id_users);

    Json(Response {
        code: Code::Success(()),
        total: None,
        data: Some(vec![id])
    })
}

#[cfg(test)]
mod route_users {

    use lib_utilities::random;
    use rocket::serde::json::Json;
    use rocket::State;

    use crate::constants::Code;
    use crate::route_auth::post as post_auth;
    use crate::route_users::*;
    use crate::struct_auth::{Auth, AuthPost};
    use crate::struct_server::Server;

    #[async_test]
    async fn test() {

        // env
        let server: Server = Server::_mock();

        /* user post as admin */
        let id_user: String = format!("{}@danavation.com", random::string(10));
        let password = random::string(10);
        let res = post(
            State::from(&server), Auth::_mock_admin(),
            Json(UserPost {
                id: id_user.clone(),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(id_user, res.data.unwrap()[0]);

        /* user post as user */
        let res = post(
            State::from(&server), Auth::_mock_user(),
            Json(UserPost {
                id: id_user.clone(),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.post] {:?}\r\n", res);
        assert_eq!(Code::Error("access-denied"), res.code);

        /* user patch as admin */
        let password0: String = random::string(10);
        let res = patch_password(
            State::from(&server), Auth::_mock_admin(),
            &id_user,
            Json(UserPatchPassword {
                password: password0.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.patch(admin)] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(id_user, res.data.unwrap()[0]);

        /* user patch as invalid user */
        let res = patch_password(
            State::from(&server), Auth::_mock_user(),
            &id_user,
            Json(UserPatchPassword {
                password: password0.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.patch(invalid-user)] {:?}\r\n", res);
        assert_eq!(Code::Error("access-denied"), res.code);

        /* user patch as valid user */
        let res = patch_password(
            State::from(&server),
            Auth{
                id: id_user.clone(),
                role: crate::constants::Role::User,
                token: "".to_string(),
            },
            &id_user,
            Json(UserPatchPassword {
                password: password0.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.patch(valid-user)] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        /* try new password */
        let res = post_auth(
            State::from(&server),
            Json(AuthPost{
                id: id_user.clone(),
                password: password0,
            }),
        ).await.into_inner();
        println!("[route_auth.post] {:?}\r\n", res);

        /* users get before delete */
        let res = get(
            State::from(&server), Auth::_mock_admin(),
            "",
            0,
            1,
        ).await.into_inner();
        println!("[route_user.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(3, res.total.unwrap());
        let users_count_before_delete: usize = res.total.unwrap();

        /* user delete and receive user id */
        let route_users_delete = delete(
            State::from(&server), Auth::_mock_admin(),
            &id_user
        ).await.into_inner();
        println!("[route_users.delete] {:?}\r\n", route_users_delete);
        assert_eq!(Code::Success(()), route_users_delete.code);
        assert_eq!(id_user, route_users_delete.data.unwrap()[0]);

        /* users get count after delete */
        let res = get(
            State::from(&server), Auth::_mock_admin(),
            "",
            0,
            10,
        ).await.into_inner();
        println!("[route_users.get] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);
        assert_eq!(res.total.unwrap() + 1, users_count_before_delete);

        /* test invalid user id create */
        let res = post(
            State::from(&server), Auth::_mock_admin(),
            Json(UserPost {
                id: "".to_string(),
                password: random::string(10),
            }),
        ).await.into_inner();
        println!("[route_users.post] {:?}\r\n", res);
        assert_eq!(Code::Error("invalid-user-id"), res.code);
    }
}
