use crate::api::auth::Auth;
use serde::Serialize;

use super::response::ServerResponse;

#[derive(Debug, Serialize)]
pub struct ChangeBalance {
    pub auth: Auth,
    pub player_id: i32,
    pub amount: i32,
}

pub async fn change_balance(
    auth: Auth,
    player_id: i32,
    amount: i32,
) -> Result<ServerResponse, reqwest::Error> {
    let request = ChangeBalance {
        auth,
        player_id,
        amount
    };
    let client = reqwest::Client::new();
    let res = client
        // .post("https://poker.kfkorulczyk.pl/api/balance")
        .post("https://poker.kfkorulczyk.pl/api/balance")
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}

