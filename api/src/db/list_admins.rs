use crate::schema::admin::username;
use crate::db::connect::establish_connection;
use crate::db::admin::ServerResponse;
use crate::services::root::Auth;
use crate::db::admin::verify_password;
use diesel::prelude::*;

pub fn list_admins(auth: Auth) -> Result<Vec<String>, ServerResponse> {
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
