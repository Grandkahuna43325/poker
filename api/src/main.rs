mod db;
mod models;
mod schema;
mod services;

extern crate argon2;

use actix_cors::Cors; // Import the Cors middleware

use actix_web::{http, App, HttpServer};

use actix_web_lab::web::spa;

use crate::services::game::change_balance as change_balance_service;
use crate::services::player::list_players as list_players_service;
use crate::services::player::list_points as list_points_service;
use crate::services::root::add_player as add_player_service;
use crate::services::root::add_user as add_user_service;
use crate::services::root::change_password as change_password_service;
use crate::services::root::delete_user as delete_user_service;
use crate::services::root::list_admins as list_admins_service;
use crate::services::root::login as login_service;
use crate::services::root::change_player as change_player_service;
use crate::services::log::logs as logs_service;
use crate::services::log::get_logs as get_logs_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(login_service)
            .service(add_user_service)
            .service(change_password_service)
            .service(delete_user_service)
            .service(add_player_service)
            .service(list_admins_service)
            .service(list_players_service)
            .service(list_points_service)
            .service(change_balance_service)
            .service(change_player_service)
            .service(logs_service)
            .service(get_logs_service)
            .service(actix_files::Files::new("/css", "css").show_files_listing())
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
