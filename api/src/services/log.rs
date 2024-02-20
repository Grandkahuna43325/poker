use crate::services::root::Auth;
use actix_web::get;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/api/logs")]
pub async fn logs(data: web::Json<Request>) -> impl Responder {
    use crate::db::logs::logs;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let auth = data.auth;

    let res = logs(auth, data.log);


    return HttpResponse::Ok().json(res);
}

#[get("/api/logs/{player_id}")]
pub async fn get_logs(path: web::Path<(i32,)>) -> impl Responder {
    use crate::db::logs::get_logs;

    let player_id = path.into_inner().0;

    let res = get_logs(player_id);


    return HttpResponse::Ok().json(res);
}


#[derive(Debug, Deserialize)]
struct Request {
    log: Logs,
    auth: Auth
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerGameStats {
    pub id: i32,
    pub name: String,
    pub folded: bool,
    pub score: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logs {
    pub game_id: i32,
    pub playerstats: Vec<PlayerGameStats>,
}
