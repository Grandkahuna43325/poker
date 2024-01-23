use crate::{api::auth::Auth, components::game::game::Player};
use serde::Serialize;

use super::response::ServerResponse;

pub async fn list_players() -> Result<Result<Vec<Player>, ServerResponse>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        // .get("http://127.0.0.1:8080/api/list_players")
        .get("https://d9fd-188-146-95-12.ngrok-free.app/api/list_players")
        .send()
        .await?;

    let response: Result<Vec<Player>, ServerResponse> = res.json().await?;
    Ok(response)
}
