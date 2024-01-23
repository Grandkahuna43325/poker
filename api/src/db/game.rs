use crate::db::admin::verify_password;
use crate::db::admin::ServerResponse;
use crate::db::connect::establish_connection;
use crate::models::Player;
use crate::services::root::Auth;
use diesel::prelude::*;

pub fn change_balance(auth: Auth, balance: i32, p_id: i32) -> ServerResponse {
    use crate::schema::player::dsl::*;

    let i = verify_password(auth);
    match i {
        ServerResponse::Ok => {}
        _ => {
            return i;
        }
    }
    let connection = &mut establish_connection();

    let connection = match connection {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return ServerResponse::DieselError(err.to_string());
        }
    };

    let player_list = player
        .select(Player::as_select())
        .filter(id.eq(p_id))
        .load(connection);

    let player_score: i32 = match player_list {
        Ok(ok) => ok.get(0).unwrap().score,
        Err(err) => {
            println!("{err}");
            return ServerResponse::DieselError(err.to_string());
        }
    };

    let new_balance = player_score + balance;

    let player_score = diesel::update(player.filter(id.eq(p_id)))
        .set(score.eq(new_balance))
        .execute(connection);

    println!("{:?}", player_score);

    ServerResponse::Ok
}
