use crate::db::admin::verify_password;
use crate::db::admin::ServerResponse;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/api/login")]
pub async fn login(data: web::Json<Auth>) -> impl Responder {
    let data = data.0;
    println!("New connection with: {:?}", data);

    let i = verify_password(data);

    return HttpResponse::Ok().json(i);
}

#[post("/api/change_password")]
pub async fn change_password(data: web::Json<ChangePasswordInfo>) -> impl Responder {
    use crate::db::admin::change_password;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let i = change_password(data);

    match i {
        Ok(ok) => {
            if ok {
                println!("Connected!");
                return HttpResponse::Ok().json(ServerResponse::Ok);
            };
            HttpResponse::Ok().json(ServerResponse::ChangePasswordError)
        }
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/api/add_admin")]
pub async fn add_user(data: web::Json<AddAdminRequest>) -> impl Responder {
    use crate::db::admin::create_admin;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let i = create_admin(data);

    match i {
        Ok(_) => HttpResponse::Ok().json(ServerResponse::Ok),
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/api/list_admins")]
pub async fn list_admins(data: web::Json<Auth>) -> impl Responder {
    use crate::db::list_admins::list_admins as list_admins_api;

    let auth = data.0;

    println!("New connection with: {:?}", auth);
    let i = list_admins_api(auth);

    match i {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(vec!["".to_string()])
        }
    }
}

#[post("/api/delete_user")]
pub async fn delete_user(data: web::Json<DeleteUserRequest>) -> impl Responder {
    use crate::db::admin::delete_admin;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let i = delete_admin(data);

    match i {
        Ok(_) => return HttpResponse::Ok().json(ServerResponse::Ok),
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/api/add_player")]
pub async fn add_player(data: web::Json<AddPlayerRequest>) -> impl Responder {
    use crate::db::admin::create_player;

    println!("New connection with: {:?}", data);
    let i = create_player(data.0);

    match i {
        Ok(_) => {
            return HttpResponse::Ok().json(ServerResponse::Ok);
        }
        Err(err) => {
            println!("err: {err}");
            return HttpResponse::Ok().json(err);
        }
    };
}

#[post("/api/change_player")]
pub async fn change_player(data: web::Json<ChangePlayerRequest>) -> impl Responder {
    use crate::db::admin::change_player;

    println!("New connection with: {:?}", data);
    let i = change_player(data.0);

    return HttpResponse::Ok().json(i);
}

#[derive(Debug, Deserialize)]
pub struct ChangePlayerRequest {
    pub auth: Auth,
    pub player_img_url: Option<String>,
    pub player_name: Option<String>,
    pub player_score: Option<i32>,
    pub player_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordInfo {
    pub username: String,
    pub current_password: String,
    pub username_to_change: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct AddAdminRequest {
    pub auth: Auth,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub username: String,
    pub password: String,
    pub username_to_delete: String,
}

#[derive(Debug, Deserialize)]
pub struct AddPlayerRequest {
    pub auth: Auth,
    pub player_name: String,
    pub player_balance: i32,
    pub player_img: String,
}
