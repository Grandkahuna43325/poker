use crate::schema::player::name;
use crate::db::connect::establish_connection;
use crate::db::admin::ServerResponse;
use crate::services::root::Auth;
use crate::db::admin::verify_password;
use diesel::prelude::*;

pub fn change_balance(auth: Auth, balance: i32, p_id: i32) -> Result<Vec<String>, ServerResponse> {
    use crate::schema::player::dsl::*;

    let i = verify_password(auth);
    match i {
        ServerResponse::Ok => {}
        _ => {
        return Err(i);
        }
    }

    let mut user_list = Vec::<String>::new();

    let connection = &mut establish_connection()?;

    let (_, player_score) = player
        .select((id.eq(p_id), score))
        .load::<(i32, i32)>(connection);


    let player_score = match player_score {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    println!("{:?}", player_score);




    Ok(user_list)
}
