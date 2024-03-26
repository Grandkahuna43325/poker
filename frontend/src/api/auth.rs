use serde::Deserialize;
use serde::Serialize;
use crate::api::response::ServerResponse;

pub async fn check_credentials(credentials: Auth) -> Result<ServerResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/login")
        .post("https://poker.kfkorulczyk.pl/api/login")
        .json(&credentials)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}
