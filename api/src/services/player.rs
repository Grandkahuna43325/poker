use actix_web::{get, web, HttpResponse, Responder};
use crate::db::player::list_points as list_points_db;

#[get("/api/list_points")]
pub async fn list_points() -> impl Responder {

    let list = list_points_db();


    return HttpResponse::Ok().json(list)
}
