use crate::api::auth::Auth;
use crate::state::State;
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
        // .post("https://poker.kfkorulczyk.pl/api/delete_user")
        .post(format!("{}/api/delete_user", State::get_url()))
        
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}

pub async fn list_users(auth: Auth) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/list_admins", State::get_url()))
        .json(&auth)
        .send()
        .await?;

    let response: Result<Vec<String>, reqwest::Error> = res.json().await;
    response
}
