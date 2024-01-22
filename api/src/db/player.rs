use crate::db::admin::ServerResponse;
use crate::db::connect::establish_connection;
use diesel::prelude::*;

pub fn list_points() -> Result<Vec<(String, i32)>, ServerResponse> {
    use crate::schema::player::dsl::*;

    let connection = &mut establish_connection()?;

    let result = player
        .select((name, score))
        .load::<(String, i32)>(connection);

    let result = match result {
        Ok(ok) => ok,
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    };

    Ok(result)
}

