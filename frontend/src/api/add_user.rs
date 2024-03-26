use serde::Serialize;
use crate::api::response::ServerResponse;
use crate::api::auth::Auth;

#[derive(Debug, Serialize)]
pub struct AddUserRequest {
    pub auth: Auth,
    pub new_username: String,
    pub new_password: String,
}

pub async fn add_user(
    username: String,
    current_password: String,
    new_username: String,
    new_password: String,
) -> Result<ServerResponse, reqwest::Error> {
    let auth = Auth{ username, password: current_password };
    let request = AddUserRequest {
        auth,
        new_username,
        new_password,
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/add_admin")
        .post("https://poker.kfkorulczyk.pl/api/add_admin")
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}
