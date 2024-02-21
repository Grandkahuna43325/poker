use serde::Serialize;
use crate::api::response::ServerResponse;
use crate::api::auth::Auth;

#[derive(Debug, Serialize)]
pub struct AddPlayerRequest {
    pub auth: Auth,
    pub player_name: String,
    pub player_balance: i32,
    pub player_img: String,
}

pub async fn add_player(
    username: String,
    current_password: String,
    player_name: String,
    player_balance: i32,
    player_img: String
) -> Result<ServerResponse, reqwest::Error> {
    let auth = Auth{ username, password: current_password };
    let request = AddPlayerRequest {
        auth,
        player_name,
        player_balance,
        player_img
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/add_player")
        .post("http://localhost:8080/api/add_player")
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}
