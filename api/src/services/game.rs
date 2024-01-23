use diesel::deserialize::Queryable;
use serde::Serialize;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;


#[post("/api/balance")]
pub async fn change_balance() -> impl Responder {

    let res = change_balance_db();


    return HttpResponse::Ok().json(list)
}
