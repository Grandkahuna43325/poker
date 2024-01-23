use yew::prelude::*;

use crate::api::{list_players::list_players, response::ServerResponse};

use super::game::game::Player;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[derive(Debug)]
pub enum Msg {
    ResOk(Vec<Player>),
    ResErr(ServerResponse),
    ResDecodeError(String),
}

#[derive(Debug)]
pub struct ListPoints {
    players: Vec<Player>,
    error: ServerResponse,
    error_decode: String,
}

impl Component for ListPoints {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let res = list_players().await;
            match res {
                Ok(res) => match res {
                    Ok(res) => Msg::ResOk(res),
                    Err(err) => Msg::ResErr(err),
                },
                Err(err) => Msg::ResDecodeError(err.to_string()),
            }
        });

        Self {
            players: Vec::new(),
            error: ServerResponse::Ok,
            error_decode: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ResOk(res) => {
                self.players = res;
                true
            }
            Msg::ResErr(res) => {
                self.error = res;
                true
            }
            Msg::ResDecodeError(res) => {
                self.error_decode = res;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ol>
            {self.players.iter().map(|player| {
            html! {
                <li><h1>{player.name.clone()}{" "}{player.score}</h1></li>
            }
        }).collect::<Vec<Html>>()}
            </ol>
        }
    }
}
