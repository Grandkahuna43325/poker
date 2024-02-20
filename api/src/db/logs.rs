use chrono::NaiveDateTime;
use crate::db::admin::verify_password;
use crate::db::admin::ServerResponse;
use crate::db::connect::establish_connection;
use crate::services::log::Logs as LogsStruct;
use crate::services::root::Auth;
use diesel::prelude::*;

pub fn logs(auth: Auth, logs_data: LogsStruct) -> ServerResponse {
    use crate::schema::logs::dsl::*;

    let i = verify_password(auth.clone());
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

    let admin_id_from_db: i32;

    {
        use crate::schema::admin::dsl::*;

        let result = admin
            .select(id)
            .filter(username.eq(&auth.username))
            .load::<i32>(connection);

        match result {
            Ok(ok) => {
                admin_id_from_db = ok[0];
            }
            Err(err) => {
                println!("{err}");
                return ServerResponse::DieselError(err.to_string());
            }
        }
    }

    //insert log into db 
    // diesel::table! {
    //     logs (id) {
    //         id -> Int4,
    //         date -> Timestamp,
    //         log -> Text,
    //         admin_id -> Int4,
    //     }
    // }
    // pub struct Logs {
    //     pub game_id: i32,
    //     pub playerstats: Vec<PlayerGameStats, Global>,
    // } 
    // pub struct PlayerGameStats {
    //     pub id: i32,
    //     pub name: String,
    //     pub folded: bool,
    //     pub score: i32,
    // }


    let text_log = logs_data.playerstats.iter().map(|x| {
        format!("player id: {} name: {} folded: {} score: {}", x.id, x.name, x.folded, x.score)
    }).collect::<Vec<String>>().join("\n");

    println!("inserting logs: \n {text_log}");

    let res = diesel::insert_into(logs)
        .values((
                log.eq(text_log),
                admin_id.eq(admin_id_from_db),
                date.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(connection);

    match res {
        Ok(ok) => {
            println!("{ok}");
        }
        Err(err) => {
            println!("{err}");
            return ServerResponse::DieselError(err.to_string());
        }
    }



    ServerResponse::Ok
}

pub fn get_logs(player_id: i32) -> Result<Vec<(String, NaiveDateTime)>, ServerResponse> {
    use crate::schema::logs::dsl::*;

// select date, log from "logs"
// WHERE log LIKE '%player id: 3%';


    let result = logs
        .select((log, date))
        .filter(crate::schema::logs::columns::log.like(format!("%player id: {player_id} %")))
        .load::<(String, NaiveDateTime)>(&mut establish_connection()?);

    match result {
        Ok(ok) => {
            return Ok(ok)
        }
        Err(err) => {
            println!("{err}");
            return Err(ServerResponse::DieselError(err.to_string()));
        }
    }
}
