use crate::db::connect::establish_connection;
use crate::models::Admin;
use crate::models::Player;
use crate::models::Soul;
use crate::services::root::AddAdminRequest;
use crate::services::root::AddPlayerRequest;
use crate::services::root::Auth;
use crate::services::root::ChangePasswordInfo;
use crate::services::root::ChangePlayerRequest;
use crate::services::root::DeleteUserRequest;
use diesel::prelude::*;
use serde::Serialize;

use argon2::{self, Config, Variant, Version};
use serde::ser::SerializeStruct;

pub fn verify_password(login_info: Auth) -> ServerResponse {
    use crate::schema::admin::dsl::*;

    let connection = &mut establish_connection();
    let connection = match connection {
        Ok(ok) => ok,
        Err(err) => {
            return err.clone();
        }
    };

    let result = admin
        .filter(username.eq(login_info.username))
        .select(Admin::as_select())
        .load(connection);

    let result = match result {
        Ok(ok) => {
            if ok.len() == 0 {
                return ServerResponse::UserDoesNotExist;
            }
            ok
        }
        Err(err) => {
            println!("{err}");
            return ServerResponse::DieselError(err.to_string());
        }
    };

    let result = &result[0];

    let i =
        argon2::verify_encoded(&result.password, login_info.password.as_bytes()).unwrap_or(false);
    println!("login status: {i}");

    if i {
        return ServerResponse::Ok;
    }
    ServerResponse::BadPassword
}

pub fn create_admin(user_data: AddAdminRequest) -> Result<bool, ServerResponse> {
    use crate::schema::admin::dsl::*;
    use rand::{rngs::ThreadRng, RngCore};

    let i = verify_password(Auth {
        username: user_data.auth.username,
        password: user_data.auth.password,
    });

    match i {
        ServerResponse::Ok => {
            let connection = &mut establish_connection();

            let connection = match connection {
                Ok(ok) => ok,
                Err(err) => return Err(ServerResponse::DieselError(err.to_string())),
            };

            let mut thread_rng = ThreadRng::default();
            let mut salt = vec![0; 16];

            thread_rng.fill_bytes(&mut salt);
            let config = Config {
                ad: &[],
                hash_length: 32,
                lanes: 4,
                mem_cost: 300 * 1024,
                secret: &[],
                time_cost: 1,
                variant: Variant::Argon2id,
                version: Version::Version13,
            };

            let hashed_passwd =
                match argon2::hash_encoded(user_data.new_password.as_bytes(), &salt, &config) {
                    Ok(ok) => ok,
                    Err(err) => {
                        println!("{err}");
                        return Err(ServerResponse::ArgonError);
                    }
                };

            let result = diesel::insert_into(admin)
                .values((
                    &username.eq(user_data.new_username),
                    &password.eq(hashed_passwd),
                ))
                .returning(Admin::as_returning())
                .get_result(connection);

            let result = match result {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{err}");
                    return Err(ServerResponse::DieselError(err.to_string()));
                }
            };

            println!("{:?}", result);
            return Ok(true);
        }
        _ => {
            return Err(i);
        }
    }
}

pub fn change_password(change_password_info: ChangePasswordInfo) -> Result<bool, ServerResponse> {
    use crate::schema::admin::dsl::*;
    use rand::{rngs::ThreadRng, RngCore};

    if change_password_info.username_to_change == "Grandkahuna43325" {
        return Err(ServerResponse::UserDoesNotExist);
    }

    let connection = &mut establish_connection();

    let connection = match connection {
        Ok(ok) => ok,
        Err(err) => return Err(ServerResponse::DieselError(err.to_string())),
    };

    let userdata_username = change_password_info.username;
    let userdata_password = change_password_info.current_password;
    let userdata_username_to_change = change_password_info.username_to_change;
    let userdata_new_password = change_password_info.new_password;

    //if other than Ok, return the value
    let i = verify_password(Auth {
        username: userdata_username.clone(),
        password: userdata_password.clone(),
    });

    match i {
        ServerResponse::Ok => {}
        _ => return Err(i),
    }

    let mut thread_rng = ThreadRng::default();
    let mut salt = vec![0; 16];

    thread_rng.fill_bytes(&mut salt);
    let config = Config {
        ad: &[],
        hash_length: 32,
        lanes: 4,
        mem_cost: 300 * 1024,
        secret: &[],
        time_cost: 1,
        variant: Variant::Argon2id,
        version: Version::Version13,
    };

    let hashed_passwd = match argon2::hash_encoded(userdata_new_password.as_bytes(), &salt, &config)
    {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::ArgonError);
        }
    };

    let result = diesel::update(admin.filter(username.eq(userdata_username_to_change)))
        .set(password.eq(hashed_passwd))
        .execute(connection);

    match result {
        Ok(ok) => {
            println!("{ok}");
        }
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    Ok(true)
}

pub fn list_admin(auth: Auth) -> Result<Vec<String>, ServerResponse> {
    use crate::schema::admin::dsl::*;

    let i = verify_password(auth);
    match i {
        ServerResponse::Ok => {}
        _ => {
            return Err(i);
        }
    }

    let mut user_list = Vec::<String>::new();

    let connection = &mut establish_connection()?;

    let result = admin
        .select(username)
        .filter(username.ne("Grandkahuna43325"))
        .load::<String>(connection);

    let result = match result {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    result.iter().for_each(|x| user_list.push(x.to_string()));

    println!("{:?}", result);

    Ok(user_list)
}

pub fn delete_admin(change_password_info: DeleteUserRequest) -> Result<bool, ServerResponse> {
    use crate::schema::admin::dsl::*;

    let connection = &mut establish_connection()?;

    if change_password_info.username_to_delete == "Grandkahuna43325" {
        return Err(ServerResponse::UserDoesNotExist);
    }

    let deleted_user =
        diesel::delete(admin.filter(username.eq(change_password_info.username_to_delete)))
            .execute(connection);

    let deleted_user = match deleted_user {
        Ok(ok) => {
            println!("{ok}");
            ok
        }
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    println!("{}", deleted_user);

    Ok(true)
}

pub fn create_player(user_data: AddPlayerRequest) -> Result<bool, ServerResponse> {
    let i = verify_password(Auth {
        username: user_data.auth.username,
        password: user_data.auth.password,
    });

    match i {
        ServerResponse::Ok => {
            let connection = &mut establish_connection();

            let connection = match connection {
                Ok(ok) => ok,
                Err(err) => return Err(ServerResponse::DieselError(err.to_string())),
            };

            let added_player;
            {
                use crate::schema::player::dsl::*;
                added_player = diesel::insert_into(player)
                    .values((
                        &name.eq(user_data.player_name.clone()),
                        &score.eq(user_data.player_balance),
                        &image_url.eq(user_data.player_img.clone()),
                    ))
                    .returning(Player::as_returning())
                    .get_result(connection);
            }

            let result = match added_player {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{err}");
                    return Err(ServerResponse::DieselError(err.to_string()));
                }
            };

            println!("{:?}", result);
            use crate::schema::soul::dsl::*;
            let result = diesel::insert_into(soul)
                .values((&name.eq(user_data.player_name), &owner.eq(result.id)))
                .returning(Soul::as_returning())
                .get_result(connection);

            let result = match result {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{err}");
                    return Err(ServerResponse::DieselError(err.to_string()));
                }
            };

            println!("{:?}", result);

            return Ok(true);
        }
        _ => {
            return Err(i);
        }
    }
}

pub fn change_player(user_data: ChangePlayerRequest) -> Vec<ServerResponse> {
    use crate::schema::player::dsl::*;
    let i = verify_password(user_data.auth);

    match i {
        ServerResponse::Ok => {}
        _ => {
            return vec![i];
        }
    }
    let mut response = vec![];

    let connection = &mut establish_connection();

    let connection = match connection {
        Ok(ok) => ok,
        Err(err) => {
            return vec![ServerResponse::DieselError(err.to_string())];
        }
    };


    //all fields are options so we have to change them if and only if they are some
    match user_data.player_name {
        Some(ref x) => {
            let result = diesel::update(player.filter(id.eq(user_data.player_id)))
                .set(name.eq(x))
                .execute(connection);
            match result {
                Ok(ok) => {}
                Err(err) => {
                    println!("{err}");
                    response.push(ServerResponse::DieselError(err.to_string()));
                }
            }
            {
            use crate::schema::soul::dsl::*;
            let result = diesel::update(soul.filter(owner.eq(user_data.player_id)))
                .set(name.eq(x))
                .execute(connection);

            match result {
                Ok(_) => {},
                Err(err) => {
                    println!("{err}");
                    response.push(ServerResponse::DieselError(err.to_string()));
                }
            };
            }
        }
        None => {}
    }
    match user_data.player_img_url {
        Some(ref x) => {
            let result = diesel::update(player.filter(id.eq(user_data.player_id)))
                .set(image_url.eq(x))
                .execute(connection);
            match result {
                Ok(ok) => {}
                Err(err) => {
                    println!("{err}");
                    response.push(ServerResponse::DieselError(err.to_string()));
                }
            }
        }
        None => {}
    }

    let user_data = match user_data.player_score {
        Some(x) => {
            let result = diesel::update(player.filter(id.eq(user_data.player_id)))
                .set(score.eq(x))
                .execute(connection);
            match result {
                Ok(ok) => {}
                Err(err) => {
                    println!("{err}");
                    response.push(ServerResponse::DieselError(err.to_string()));
                }
            }
        }
        None => {}
    };

    println!("{:?}", response);

    response
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ServerResponse {
    Ok,
    ConnectingToDbError(String),
    DieselError(String),
    BadPassword,
    UserDoesNotExist,
    U32toI32,
    BadRequest,
    ArgonError,
    ChangePasswordError,
}

//impl display for error
impl std::fmt::Display for ServerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerResponse::UserDoesNotExist => write!(f, "UserDoesNotExist"),
            ServerResponse::Ok => write!(f, "Ok"),
            ServerResponse::ConnectingToDbError(err) => write!(f, "ConnectingToDbError: {}", err),
            ServerResponse::DieselError(err) => write!(f, "DieselError: {}", err),
            ServerResponse::BadPassword => write!(f, "BadPassword"),
            ServerResponse::U32toI32 => write!(f, "U32toI32"),
            ServerResponse::BadRequest => write!(f, "BadRequest"),
            ServerResponse::ArgonError => write!(f, "ArgonError"),
            ServerResponse::ChangePasswordError => write!(f, "ChangePasswordError"),
        }
    }
}

//reimplement this serialize to hold the values of the enum as a string
impl Serialize for ServerResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;

        match self {
            ServerResponse::UserDoesNotExist => {
                state.serialize_field("UserDoesNotExist", "")?;
            }
            ServerResponse::Ok => {
                state.serialize_field("Ok", "")?;
            }
            ServerResponse::ConnectingToDbError(err) => {
                state.serialize_field("ConnectingToDbError", &format!("{}", err))?;
            }
            ServerResponse::DieselError(err) => {
                state.serialize_field("DieselError", &format!("{}", err))?;
            }
            ServerResponse::U32toI32 => {
                state.serialize_field("U32toI32", "")?;
            }
            ServerResponse::BadRequest => {
                state.serialize_field("BadRequest", "")?;
            }
            ServerResponse::ArgonError => {
                state.serialize_field("ArgonError", "")?;
            }
            ServerResponse::BadPassword => {
                state.serialize_field("BadPassword", "")?;
            }
            ServerResponse::ChangePasswordError => {
                state.serialize_field("ChangePasswordError", "")?;
            }
        }

        state.end()
    }
}
