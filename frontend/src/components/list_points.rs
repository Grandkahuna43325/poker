use yew::{prelude::*, virtual_dom::VNode};

use crate::api::{list_players::list_players, response::ServerResponse};

use super::game::game::Player;


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
    current_player_id: i32,
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
            current_player_id: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.players.len() == 0 {
            return html! {<div>{"Pobieranie graczy..."}</div>};
        }
        if self.players.len() < 3 {
            return html! {<div>{"not enough players"}</div>};
        }
        match self.error {
            ServerResponse::Ok => {}
            _ => return html! {<div>{"Error: "}</div>},
        }

        let top_3 = self.players.iter().take(3).map(|p| p.clone());

        let rest = self.players.iter().enumerate().skip(3).map(|(i, p)| {
            let lost = p.score < 0;
            let id = p.id;
            html! {
                <a>
                  <div id="item">
                      <div class="num">{i+1}</div>
                      <img src={p.image_url.clone()}
                      alt={p.name.clone()} />
                      <div class="name">{p.name.clone()}</div>
                      <div class="score" style={if lost {"color: red"} else {""}}>{p.score}</div>
                  </div>
                </a>
            }
        }).collect::<Vec<VNode>>();

        let p1_id = top_3.clone().nth(0).unwrap().id;
        let p2_id = top_3.clone().nth(1).unwrap().id;
        let p3_id = top_3.clone().nth(2).unwrap().id;

        html! {
          <div>
              <link rel="stylesheet" type="text/css" href="https://poker.kfkorulczyk.pl/css/leaderboard.css"/>
                <div id="leaderboard">
                  <div>
                    <div id="top_3">
                      <div class="top two">
                        <img
                          src={top_3.clone().nth(1).unwrap().image_url.clone()}
                          alt=""
                        />
                        <div>{"2"}</div>
                        <div>{top_3.clone().nth(1).unwrap().name.clone()}</div>
                        <div>{top_3.clone().nth(1).unwrap().score}</div>
                      </div>
                      <div class="top one">
                        <img
                          src={top_3.clone().nth(0).unwrap().image_url.clone()}
                          alt={top_3.clone().nth(0).unwrap().name.clone()}
                        />
                        <div>{"1"}</div>
                        <div>{top_3.clone().nth(0).unwrap().name.clone()}</div>
                        <div>{top_3.clone().nth(0).unwrap().score}</div>
                      </div>
                      <div class="top three">
                        <img
                          src={top_3.clone().nth(2).unwrap().image_url.clone()}
                          alt={top_3.clone().nth(2).unwrap().name.clone()}
                        />
                        <div>{"3"}</div>
                        <div>{top_3.clone().nth(2).unwrap().name.clone()}</div>
                        <div>{top_3.clone().nth(2).unwrap().score}</div>
                      </div>
                    </div>

                    </div>
                    <div id="list">
                    {rest}
                  </div>
                </div>
        </div>
        }
    }
}
