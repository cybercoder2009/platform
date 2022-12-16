use rocket::State;
use rocket::serde::json::Json;
use lib_utilities::hash::sha3_256;
use lib_utilities::validate::email;

use crate::constants::{Code, error};
use crate::struct_db::key_user;
use crate::struct_auth::{Auth, AuthPost};
use crate::struct_response::Response;
use crate::struct_server::Server;
use crate::struct_user::User;

#[post("/auth", format = "application/json", data = "<auth>")]
pub async fn post<'r>(
    server: &'r State<Server>,
    auth: Json<AuthPost>
) -> Json<Response<Auth>> {
    
    // filters
    if !email(&auth.id) { return error("invalid-email"); }
    let _key_user: String = key_user(&auth.id);
    let opt_user: Option<User> = server.db.read::<User>(&_key_user);
    if opt_user.is_none() { return error("user-not-found"); }
    
    let user: User = opt_user.unwrap();
    if user.password == sha3_256(&auth.password) {
        let auth: Auth = Auth {
            id: auth.id.clone(),
            role: user.role.clone(),
            token: user.token.clone(),
        };   
        { server.cache.lock().unwrap().put(user.token.clone(), auth.clone()); }
        Json(Response {
            code: Code::Success(()),
            total: None,
            data: Some(vec![auth])
        })
    } else {
        error("invalid-password")
    }
}

#[cfg(test)]
mod route_auth {

    use rocket::State;
    use rocket::serde::json::Json;
    use lib_utilities::random;
    
    use crate::constants::Code;
    use crate::route_auth::post as post_auth;
    use crate::route_users::post;
    use crate::struct_auth::{Auth, AuthPost};
    use crate::struct_server::Server;
    use crate::struct_user::UserPost;

    #[async_test]
    async fn test() {

        // env
        let server: Server = Server::_mock();

        /* users post */
        let id: String = format!("{}@danavation.com", random::string(10));
        let password = random::string(10);
        let res_user_post = post(
            State::from(&server), Auth::_mock_admin(),
            Json(UserPost {
                id: id.clone(),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_users.post] {:?}\r\n", res_user_post);

        /* auth post with correct id and password, receive token */
        let res = post_auth(
            State::from(&server),
            Json(AuthPost {
                id: id.clone(),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_auth.post] {:?}\r\n", res);
        assert_eq!(Code::Success(()), res.code);

        /* login with invalid id */
        let res = post_auth(
            State::from(&server),
            Json(AuthPost {
                id: random::string(10),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_auth.post] {:?}\r\n", res);
        assert_eq!(Code::Error("invalid-email"), res.code);

        /* auth post with unregistered user */
        let res = post_auth(
            State::from(&server),
            Json(AuthPost {
                id: format!("{}@reducing.ca", random::string(10)),
                password: password.clone(),
            }),
        ).await.into_inner();
        println!("[route_auth.post] {:?}\r\n", res);
        assert_eq!(Code::Error("user-not-found"), res.code);

        /* auth post with wrong password */
        let res = post_auth(
            State::from(&server),
            Json(AuthPost {
                id: id.clone(),
                password: random::string(10),
            }),
        ).await.into_inner();
        println!("[route_auth.post] {:?}\r\n", res);
        assert_eq!(Code::Error("invalid-password"), res.code);
    }
}
