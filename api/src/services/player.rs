use crate::db::list_players::list_players as list_players_db;
use crate::db::player::list_points as list_points_db;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::deserialize::Queryable;
use serde::Deserialize;
use serde::Serialize;

use super::root::Auth;

#[get("/api/list_points")]
pub async fn list_points() -> impl Responder {
    let list = list_points_db();

    return HttpResponse::Ok().json(list);
}

#[get("/api/list_players")]
pub async fn list_players() -> impl Responder {
    println!("list_players called");
    let players = list_players_db();
    return HttpResponse::Ok().json(players);
}

#[derive(Queryable, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub score: i32,
    pub image_url: String,
}

