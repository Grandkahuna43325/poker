use crate::api::response::ServerResponse;
use crate::api::auth::Auth;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChangePlayerRequest {
    auth: Auth,
    player_img_url: Option<String>,
    player_name: Option<String>,
    player_score: Option<i32>,
    player_id: i32,
}

pub async fn change_player(
    username: String,
    password: String,
    player_img_url: Option<String>,
    player_name: Option<String>,
    player_score: Option<i32>,
    player_id: i32,

) -> Result<Vec<ServerResponse>, reqwest::Error> {
    let auth = Auth {
        username,
        password,
    };
    let request = ChangePlayerRequest {
        auth,
        player_img_url,
        player_name,
        player_score,
        player_id,
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/change_password")
        .post("https://poker.kfkorulczyk.pl/api/change_player")
        .json(&request)
        .send()
        .await?;

    let response: Result<Vec<ServerResponse>, reqwest::Error> = res.json().await;

    response
}
