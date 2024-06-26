use crate::api::response::ServerResponse;
use crate::api::auth::Auth;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChangePasswordInfo {
    auth: Auth,
    pub username_to_change: String,
    pub new_password: String,
}

pub async fn change_password(
    username: String,
    current_password: String,
    username_to_change: String,
    new_password: String,
) -> Result<ServerResponse, reqwest::Error> {
    let auth = Auth {
        username,
        password: current_password,
    };
    let request = ChangePasswordInfo {
        auth,
        username_to_change,
        new_password,
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("https://poker.kfkorulczyk.pl/api/change_password")
        .post("https://poker.kfkorulczyk.pl/api/change_password")
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}
