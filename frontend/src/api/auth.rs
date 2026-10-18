use serde::Deserialize;
use serde::Serialize;
use crate::api::response::ServerResponse;
use crate::state::State;

pub async fn check_credentials(credentials: Auth) -> Result<ServerResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        // .post("https://poker.kfkorulczyk.pl/api/login")
        .post(format!("{}/api/login", State::get_url()))
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
