mod db;
mod models;
mod schema;
mod services;

extern crate argon2;

use actix_cors::Cors; // Import the Cors middleware

use actix_web::{http, App, HttpServer};

use crate::services::root::add_user as add_user_service;
use crate::services::root::login as login_service;
use crate::services::root::change_password as change_password_service;
use crate::services::root::delete_user as delete_user_service;
use crate::services::root::add_player as add_player_service;
use crate::services::player::list_points as list_points_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(login_service)
            .service(add_user_service)
            .service(change_password_service)
            .service(delete_user_service)
            .service(add_player_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
