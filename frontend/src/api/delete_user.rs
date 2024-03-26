use crate::api::auth::Auth;
use serde::Serialize;

use super::response::ServerResponse;

#[derive(Debug, Serialize)]
pub struct DeleteUserRequest {
    pub username: String,
    pub password: String,
    pub username_to_delete: String,
}

pub async fn delete_user(
    username: String,
    password: String,
    username_to_delete: String,
) -> Result<ServerResponse, reqwest::Error> {
    let request = DeleteUserRequest {
        username,
        password,
        username_to_delete,
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/delete_user")
        .post("https://poker.kfkorulczyk.pl/api/delete_user")
        
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}

pub async fn list_users(auth: Auth) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/api/list_admins")
        .json(&auth)
        .send()
        .await?;

    let response: Result<Vec<String>, reqwest::Error> = res.json().await;
    response
}
