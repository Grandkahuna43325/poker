use crate::services::root::Auth;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/api/balance")]
pub async fn change_balance(data: web::Json<ChangeBalance>) -> impl Responder {
    use crate::db::game::change_balance as change_balance_db;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let auth = data.auth;
    let amount = data.amount;
    let player_id = data.player_id;

    let res = change_balance_db(auth, amount, player_id);

    return HttpResponse::Ok().json(res);
}

#[derive(Debug, Deserialize)]
pub struct ChangeBalance {
    pub auth: Auth,
    pub player_id: i32,
    pub amount: i32,
}
