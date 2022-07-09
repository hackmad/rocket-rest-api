use super::models::*;
use crate::db::MyDatabase;
use rocket::response::status::{BadRequest, Forbidden};
use rocket::serde::json::Json;
use rocket::{get, post};

#[get("/users", format = "application/json")]
pub async fn get_all(conn: MyDatabase) -> Json<Vec<UserResponse>> {
    conn.run(|c| Json(User::get_all_users(c))).await
}

#[post("/users", format = "application/json", data = "<new_user>")]
pub async fn new_user(
    conn: MyDatabase,
    new_user: Json<NewUserRequest>,
) -> Result<Json<UserResponse>, BadRequest<String>> {
    let username = new_user.username.clone();
    let result = conn
        .run(|c| User::insert_user(new_user.into_inner(), c))
        .await;
    match result {
        Ok(_) => {
            let user = conn
                .run(move |c| User::get_user_by_username(username.as_str(), c))
                .await;
            match user {
                Some(u) => Ok(Json(u)),
                None => Err(BadRequest(Some("insert failed".to_string()))),
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(BadRequest(Some(e.to_string())))
        }
    }
}

#[get("/users/<username>", format = "application/json")]
pub async fn find_user(conn: MyDatabase, username: String) -> Option<Json<UserResponse>> {
    conn.run(move |c| User::get_user_by_username(username.as_str(), c).map(|user| Json(user)))
        .await
}

#[post("/login", format = "application/json", data = "<creds>")]
pub async fn login(
    conn: MyDatabase,
    creds: Json<LoginRequest>,
) -> Result<Json<UserResponse>, Forbidden<String>> {
    let result = conn.run(|c| User::login(creds.into_inner(), c)).await;
    match result {
        Some(user) => Ok(Json(user)),
        None => Err(Forbidden(Some(
            "Invalid username and/or password".to_string(),
        ))),
    }
}

use crate::server;
use rocket::{Build, Rocket};

#[allow(unused)]
fn rocket() -> Rocket<Build> {
    server::build()
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn get_all_users_returns_empty_when_no_users_exist() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/v1/users").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().unwrap();
        assert_eq!(body, "[]".to_string());
    }
}
