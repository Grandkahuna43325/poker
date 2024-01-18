use crate::db::admin::verify_password;
use crate::db::admin::ServerResponse;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/api/login")]
pub async fn login(data: web::Json<Auth>) -> impl Responder {
    let data = data.0;
    println!("New connection with: {:?}", data);

    let i = verify_password(data);

    return HttpResponse::Ok().json(true);
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

#[post("/api/add_user")]
pub async fn add_user(data: web::Json<AddUserRequest>) -> impl Responder {
    use crate::db::admin::create_user;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let i = create_user(data);

    match i {
        Ok(_) => HttpResponse::Ok().json(ServerResponse::Ok),
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/api/delete_user")]
pub async fn delete_user(data: web::Json<DeleteUserRequest>) -> impl Responder {
    use crate::db::admin::delete_user;

    let data = data.0;

    println!("New connection with: {:?}", data);
    let i = delete_user(data);

    match i {
        Ok(_) => return HttpResponse::Ok().json(ServerResponse::Ok),
        Err(err) => {
            println!("err: {err}");
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/api/add_player")]
pub async fn add_player() -> impl Responder {
    use crate::db::admin::create_user;

    HttpResponse::Ok().json(ServerResponse::Ok)
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
pub struct AddUserRequest {
    pub username: String,
    pub current_password: String,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub username: String,
    pub password: String,
    pub username_to_delete: String,
}
