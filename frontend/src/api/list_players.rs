use crate::components::game::game::Player;

use super::response::ServerResponse;

pub async fn list_players() -> Result<Result<Vec<Player>, ServerResponse>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        // .get("https://poker.kfkorulczyk.pl/api/list_players")
        .get("https://poker.kfkorulczyk.pl/api/list_players")
        .send()
        .await?;

    let response: Result<Vec<Player>, ServerResponse> = res.json().await?;
    Ok(response)
}
