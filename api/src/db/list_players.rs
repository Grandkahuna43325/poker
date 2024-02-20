use crate::db::admin::ServerResponse;
use crate::db::connect::establish_connection;
use crate::services::player::Player;
use diesel::prelude::*;

pub fn list_players() -> Result<Vec<Player>, ServerResponse> {
    use crate::schema::player::dsl::*;

    let mut player_vec = Vec::<Player>::new();

    let connection = &mut establish_connection()?;

    //order players by score
    let result = player
        .order_by(score.desc())
        .load::<Player>(connection);

    let result = match result {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    result.iter().for_each(|x| {
        let x = x.clone();
        player_vec.push(x)
    });

    Ok(player_vec)
}
