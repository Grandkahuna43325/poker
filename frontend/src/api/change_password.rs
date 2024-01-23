use crate::api::response::ServerResponse;
use crate::api::auth::Auth;
use serde::Serialize;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}
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
        // .post("http://127.0.0.1:8080/api/change_password")
        .post("https://d9fd-188-146-95-12.ngrok-free.app/api/change_password")
        .json(&request)
        .send()
        .await?;

    let response: Result<ServerResponse, reqwest::Error> = res.json().await;

    response
}
