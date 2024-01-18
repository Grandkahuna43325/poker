use crate::db::admin::ServerResponse;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, ServerResponse> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let result = PgConnection::establish(&database_url);
    match result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(ServerResponse::ConnectingToDbError(err.to_string())),
    }
}
