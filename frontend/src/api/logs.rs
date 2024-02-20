use chrono::NaiveDateTime;
use serde::Serialize;
use crate::components::game::game::Logs;

use super::{response::ServerResponse, auth::Auth};

pub async fn add_logs(log: Logs, auth: Auth) -> Result<ServerResponse, reqwest::Error> {
    let req =  Request {
        log,
        auth
    };

    let client = reqwest::Client::new();
    let res = client
        // .get("http://127.0.0.1:8080/api/list_players")
        .post("http://localhost:8080/api/logs")
        .json(&req)
        .send()
        .await?;

    let response: ServerResponse = res.json().await?;
    Ok(response)
}

pub async fn get_logs(player_id: i32) -> Result<Result<Vec<(String, NaiveDateTime)>, ServerResponse>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://127.0.0.1:8080/api/logs/{player_id}"))
        .send()
        .await?;

    let response: Result<Vec<(String, NaiveDateTime)>, ServerResponse> = res.json().await?;
    Ok(response)
}

#[derive(Debug, Serialize)]
struct Request {
    log: Logs,
    auth: Auth
}
