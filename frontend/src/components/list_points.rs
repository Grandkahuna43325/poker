use regex::Regex;
use chrono::NaiveDateTime;
use yew::{prelude::*, virtual_dom::VNode};

use crate::api::{list_players::list_players, logs::get_logs, response::ServerResponse};

use super::game::game::Player;


#[derive(Debug)]
pub enum Msg {
    ResOk(Vec<Player>),
    ResOkLogs(Vec<(String, NaiveDateTime)>),
    ResErr(ServerResponse),
    ResDecodeError(String),
    GetLogs(i32),
    Action(Action),
}

#[derive(Debug)]
pub enum Action {
    None,
    ViewLogs,
    SelectPlayer(i32),
}

#[derive(Debug)]
pub struct ListPoints {
    players: Vec<Player>,
    error: ServerResponse,
    error_decode: String,
    logs: Vec<(String, NaiveDateTime)>,
    action: Action,
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
            logs: Vec::new(),
            action: Action::None,
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
            Msg::GetLogs(id) => {
                ctx.link().send_future(async move {
                    let result = get_logs(id).await;
                    match result {
                        Ok(res) => match res {
                            Ok(res) => Msg::ResOkLogs(res),
                            Err(err) => Msg::ResErr(err),
                        },
                        Err(err) => Msg::ResDecodeError(err.to_string()),
                    }
                });
                true
            }
            Msg::ResOkLogs(res) => {
                self.logs = res;

                true
            }
            Msg::Action(action) => {
                self.action = action;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.action {
            Action::None => {}
            Action::ViewLogs => {
                let logs = self.logs.clone();
                let logs = logs.into_iter().map(|(log, time)| {
                    let time = time.format("%Y-%m-%d %H:%M:%S").to_string();
                    let re = Regex::new(r"name: (\w+) folded: (\w+) score: (-?\d+)").unwrap();

                    let result = re.captures_iter(&log)
                        .map(|cap| html!{
                            <div>
                            {format!("{} folded: {} score: {}", &cap[1], &cap[2], &cap[3])}
                            <br/>
                            </div>
                        })
                        .collect::<Vec<VNode>>();
                    html! {
                        <div>
                            <div>{time}</div>
                            <div>{result}</div>
                            <br/>
                        </div>
                    }
                }).collect::<VNode>();
                return html! {
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::Action(Action::None))}>{"Back"}</button>
                        <div>
                            { logs }
                        </div>
                    </div>
                };
            }
            Action::SelectPlayer(id) => {
                ctx.link().send_message(Msg::Action(Action::ViewLogs));
                ctx.link().send_message(Msg::ResOkLogs(Vec::new()));
                ctx.link().send_message(Msg::GetLogs(id));
            }
        }

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
                <a
                onclick={ctx.link().callback(move |_| Msg::Action(Action::SelectPlayer(id)))}>
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
              <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/leaderboard.css"/>
                <div id="leaderboard">
                  <div>
                    <div id="top_3">
                      <div class="top two" onclick={ctx.link().callback(move |_| Msg::Action(Action::SelectPlayer(p2_id)))}>
                        <img
                          src={top_3.clone().nth(1).unwrap().image_url.clone()}
                          alt=""
                        />
                        <div>{"2"}</div>
                        <div>{top_3.clone().nth(1).unwrap().name.clone()}</div>
                        <div>{top_3.clone().nth(1).unwrap().score}</div>
                      </div>
                      <div class="top one" onclick={ctx.link().callback(move |_| Msg::Action(Action::SelectPlayer(p1_id)))}>
                        <img
                          src={top_3.clone().nth(0).unwrap().image_url.clone()}
                          alt={top_3.clone().nth(0).unwrap().name.clone()}
                        />
                        <div>{"1"}</div>
                        <div>{top_3.clone().nth(0).unwrap().name.clone()}</div>
                        <div>{top_3.clone().nth(0).unwrap().score}</div>
                      </div>
                      <div class="top three" onclick={ctx.link().callback(move |_| Msg::Action(Action::SelectPlayer(p3_id)))}>
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
