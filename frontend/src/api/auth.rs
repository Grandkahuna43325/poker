use serde::Deserialize;
use serde::Serialize;
use crate::api::response::ServerResponse;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

pub async fn check_credentials(credentials: Auth) -> Result<ServerResponse, reqwest::Error> {
    log!("{}", format!("{:?}", credentials));
    let client = reqwest::Client::new();
    let res = client
        // .post("http://127.0.0.1:8080/api/login")
        .post("https://d9fd-188-146-95-12.ngrok-free.app/api/login")
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
