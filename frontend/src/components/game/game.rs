use yew::html::IntoPropValue;
use yew::virtual_dom::VNode;
use serde::Serialize;
use yew::prelude::*;

use crate::api::auth::Auth;
use crate::api::game::change_balance;
use crate::api::response::ServerResponse;
use crate::api::list_players::list_players;

use serde::Deserialize ;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[derive(Debug)]
pub enum Msg {
    Raise(i32),
    AllIn,
    Reset,
    Next,
    Action(Action),
    PlayerList(Vec<Player>),
    ServerError(ServerResponse),
    DecodeError(String),
    AddPlayerById(i32),
    RemovePlayerById(i32),
    FoldById(i32),
    None
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub username: String,
    pub password: String
}

#[derive(Debug)]
enum GameStage {
    Flop,
    Turn,
    River,
}

#[derive(Debug)]
pub enum Action {
    None,
    Error(ServerResponse),
    ErrorDecode(String),
    AddPlayer,
    DeletePlayer,
    GetPlayerBalance
}

#[derive(Debug)]
pub struct Game {
    pot_size: i32,
    players: Vec<Player>,
    all_players: Vec<Player>,
    stage: GameStage,
    action: Action,
    folded: Vec<i32>
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let result = list_players().await;
            log!("{:?}", result);
            match result {
                Ok(res) => match res {
                    Ok(res) => Msg::PlayerList(res),
                    Err(err) => Msg::ServerError(err),
                },
                Err(err) => Msg::DecodeError(err.to_string()),
           }
        });

        let username = ctx.props().username.clone();
        let password = ctx.props().password.clone();

        Self {
            pot_size: 0,
            players: Vec::new(),
            stage: GameStage::Flop,
            all_players: Vec::new(),
            action: Action::None,
            folded: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => {false},
            Msg::Raise(amount) => {
                self.pot_size += amount;
                true
            }
            Msg::AllIn => {
                self.pot_size = 100000;
                true
            }
            Msg::Reset => {
                self.pot_size = 0;
                true
            }
            Msg::Next => {
                self.stage = match self.stage {
                    GameStage::Flop => GameStage::Turn,
                    GameStage::Turn => GameStage::River,
                    GameStage::River => {
                        self.pot_size = 0;
                        self.folded = vec![];
                        self.action = Action::None;
                        GameStage::Flop
                    }
                };
                true
            }
            Msg::Action(action) => {
                self.action = action;
                true
            }
            Msg::PlayerList(players) => {
                self.all_players = players;
                true
            }
            Msg::ServerError(err) => {
                log!("{}", err);
                self.action = Action::Error(err);
                true
            }
            Msg::DecodeError(err) => {
                log!("{}", err);
                self.action = Action::ErrorDecode(err);
                true
            }
            Msg::AddPlayerById(id) => {
                let player = self.all_players
                    .iter()
                    .find(|p| p.id == id)
                    .unwrap();

                self.players
                    .push(player.clone());


                self.action = Action::None;
                true
            }
            Msg::RemovePlayerById(id) => {
                self.players.retain(|p| p.id != id);
                self.action = Action::None;

                true
            }
            Msg::FoldById(id) => {
                self.folded.push(id);
                let username = ctx.props().username.clone();
                let password = ctx.props().password.clone();
                let amount = self.pot_size;
                log!("folded on {}", self.pot_size);

                ctx.link().send_future(async move {
                    let result = change_balance(
                        Auth { username, password },
                        id, amount
                        ).await;
                    log!("{:?}", result);
                    Msg::None
                });

                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.action {
            Action::None => {}
            Action::AddPlayer => {
                return html! {<h1>{
                    self.all_players.iter().map(|p| {
                        let (id, name) = (p.id, p.name.clone());

                        html!
                        {<p onclick={ctx.link().callback(move |_| Msg::AddPlayerById(id))}>{name}</p>}

                    }).collect::<Vec<VNode>>()
                }</h1>};
            }
            Action::DeletePlayer => {
                return html! {<h1>{
                    self.players.iter().map(|p| {
                        let (id, name) = (p.id, p.name.clone());

                        html!
                        {<p onclick={ctx.link().callback(move |_| Msg::RemovePlayerById(id))}>{name}</p>}

                    }).collect::<Vec<VNode>>()
                }</h1>};
            }
            Action::GetPlayerBalance => {
                return html! {<h1>{"get player balance"}</h1>};
            }
            Action::Error(err) => {
                log!("error: {:?}", err);
                return html! {<h1>{"error"}</h1>};
            }
            Action::ErrorDecode(err) => {
                log!("error: {:?}", err);
                return html! {<h1>{"error"}</h1>};
            }

        }

        let max_players=27;
        if self.players.len() > max_players {
            return html! {<h1>{"too many players"}</h1>};
        }

        let mut playernode: Vec<VNode> =  self.players.iter().map(|p| {
            let (id, name, url) = (p.id, p.name.clone(), p.image_url.clone());
            let folded: bool = self.folded.contains(&id);
            html! {
                <div class="circle-box">
                    <p onclick={ctx.link().callback(move |_| Msg::FoldById(id))}>
                      <img class="circle" src={url} style={if folded {"opacity: 0.5"} else {""}}/>
                    </p>
                </div>
            }
        }).collect();


        playernode.resize_with(max_players, || {
            html! {
                <div class="circle-box">
                    <img class="circle" />
                </div>
            }
        });



        html! {
            <>
                <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/game.css"/>
                <div id="circles-top">
                {
                    playernode[0..=5].iter().cloned().collect::<Html>()
                }
                </div>
                <div id="circles-left">
                  {playernode[6..=13].iter().cloned().collect::<Html>()}
                  // <div class="circle-box">
                  //   <img class="circle" id="player6"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player7"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player8"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player9"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player10"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player11"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player12"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player13"/>
                  // </div>
                </div>
                <div id="circles-bottom">
                {playernode[14..=18].iter().cloned().collect::<Html>()}
                  // <div class="circle-box">
                  //   <img class="circle" id="player14"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player15"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player16"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player17"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player18"/>
                  // </div>
                </div>
                <div id="circles-right">
                {playernode[19..=26].iter().cloned().collect::<Html>()}
                  // <div class="circle-box">
                  //   <img class="circle" id="player19"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player20"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player21"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player22"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player23"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player24"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player25"/>
                  // </div>
                  // <div class="circle-box">
                  //   <img class="circle" id="player26"/>
                  // </div>
                </div>
                <div id="table">
                  <div id="table_top">
                    <div id="options">
                      <div id="buttons">
                          <button id="delete" onclick={ctx.link().callback(|_| Msg::Action(Action::DeletePlayer))}>{ "delete player" }</button>
                          <button id="add" onclick={ctx.link().callback(|_| Msg::Action(Action::AddPlayer))}>{ "add player" }</button>
                      </div>
                      </div>
                  </div>
                  <div id="table_bottom">
                    <h1 id="pot">{ self.pot_size }</h1>
                    <div id="bets">
                      <div class="left">
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Raise(1))}>{"1"}</button>
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Raise(5))}>{"5"}</button>
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Raise(10))}>{"10"}</button>
                      </div>

                      <div class="right">
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Raise(100))}>{"100"}</button>
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Raise(1000))}>{"1000"}</button>
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::AllIn)}>{"All in"}</button>
                      </div>
                    </div>
                  </div>
                        <button class="raise" onclick={ctx.link().callback(|_| Msg::Reset)}>{"reset"}</button>
                  <div>
                    <h2>{format!("{:?}", self.stage)}</h2>
                    <button id="next" onclick={ctx.link().callback(|_| Msg::Next)}>{"next round"}</button>
                  </div>
                </div>
            </>
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub score: i32,
    pub image_url: String,
}
