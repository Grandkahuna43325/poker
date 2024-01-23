use crate::{api::{add_player::add_player, list_players::list_players, response::ServerResponse}, router::AdminRoute};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

use super::game::game::Player;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[derive(Debug)]
pub enum Msg {
    AddUser,
    ChangeUser,
    RemoveUser,
    ChooseAction,
}

pub struct MainPanel {
    username: String,
    password: String,
    action: Msg,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub username: String,
    pub password: String,
}

impl Component for MainPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if ctx.props().username.is_empty() && ctx.props().password.is_empty() {
            ctx.link()
                .navigator()
                .unwrap()
                .push(&AdminRoute::AdminPanel);
        }

        Self {
            username: ctx.props().username.clone(),
            password: ctx.props().password.clone(),
            action: Msg::ChooseAction,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddUser => {
                self.action = Msg::AddUser;
                true
            }
            Msg::ChangeUser => {
                self.action = Msg::ChangeUser;
                true
            }
            Msg::RemoveUser => {
                self.action = Msg::RemoveUser;
                true
            }
            Msg::ChooseAction => {
                self.action = Msg::ChooseAction;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_post = ctx.link().callback(|_| Msg::AddUser);
        let remove_post = ctx.link().callback(|_| Msg::RemoveUser);
        let change_post = ctx.link().callback(|_| Msg::ChangeUser);

        match self.action {
            Msg::AddUser => html! {
                html!{
                    <AddUser username={self.username.clone()} password={self.password.clone()}/>
                }
            },
            Msg::ChangeUser => html! {
                <ChangeUser username={self.username.clone()} password={self.password.clone()}/>
            },
            Msg::RemoveUser => html! {
                <h1>{"usuwanie gracza"}</h1>
            },
            Msg::ChooseAction => html! {
                <div class="admin-panel">
                    <link rel="stylesheet" type="text/css" href="https://d9fd-188-146-95-12.ngrok-free.app/css/admin_panel.css"/>
                    <h2 style="color: black;">{"wybierz opcję"}</h2>

                    <button class="option add-post" onclick={add_post} >{"dodaj gracza"}</button>
                    <button class="option remove-post" onclick={remove_post}>{"usuń gracza"}</button>
                    <button class="option change-post" onclick={change_post}>{"zmien gracza"}</button>
                </div>
            },
        }
    }
}

pub enum AdduserMsg {
    Add,
    NewName(String),
    NewImgUrl(String),
    NewScore(i32),
    Success,
    Error(String),
}

pub struct AddUser {
    username: String,
    password: String,
    player_img_url: String,
    player_name: String,
    player_score: i32,
    success: bool,
    error: String,
}

impl Component for AddUser {
    type Message = AdduserMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let username = ctx.props().username.clone();
        let password = ctx.props().password.clone();
        Self {
            username,
            password,
            player_img_url: String::new(),
            player_name: String::new(),
            player_score: 1100,
            success: false,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AdduserMsg::Add => {
                log!("Add");
                let username = self.username.clone();
                let password = self.password.clone();
                let name = self.player_name.clone();
                let image_url = self.player_img_url.clone();
                let score = self.player_score;
                ctx.link().send_future(async move {
                    let result = add_player(username, password, name, score, image_url).await;
                    log!("{:?}", result);
                    match result {
                        Ok(_) => AdduserMsg::Success,
                        Err(err) => AdduserMsg::Error(err.to_string()),
                    }
                });
                true
            }
            AdduserMsg::NewName(username) => {
                self.player_name = username;
                true
            }
            AdduserMsg::NewImgUrl(player_img_url) => {
                self.player_img_url = player_img_url;
                true
            }
            AdduserMsg::NewScore(player_score) => {
                self.player_score = player_score;
                true
            }
            AdduserMsg::Success => {
                self.success = true;
                true
            }
            AdduserMsg::Error(err) => {
                self.error = err;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.success {
            return html! {
                <h1>{"Udało się!"}</h1>
            };
        }
        if self.error.len() > 0 {
            return html! {
                <h1>{self.error.clone()}</h1>
            };
        }

        let add_user = ctx.link().callback(|_| AdduserMsg::Add);

        let new_name = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            AdduserMsg::NewName(value)
        });

        let new_img = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            AdduserMsg::NewImgUrl(value)
        });

        let new_score = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse()
                .unwrap();
            AdduserMsg::NewScore(value)
        });

        html! {
            <div class="add-user">
                <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/admin_panel.css"/>
                <h2>{"dodaj użytkownika"}</h2>
                <input
                    type="text"
                    placeholder="nazwa gracza"
                    oninput={new_name}
                />
                <input
                    type="text"
                    placeholder="url obrazka"
                    oninput={new_img}
                />
                <input
                    type="number"
                    value={self.player_score.clone().to_string()}
                    oninput={new_score}
                />
                <button class="add-user-button" onclick={add_user} disabled={self.player_img_url.is_empty() || self.player_name.is_empty()}>{"dodaj"}</button>
            </div>
        }
    }
}

pub enum ChangeUserMsg {
    NewName(String),
    NewImgUrl(String),
    NewScore(i32),
    Success(Vec<ServerResponse>),
    Error(String),
    Change,
    PlayerList(Vec<Player>),
    NewId(i32),
}


pub struct ChangeUser {
    username: String,
    password: String,
    user_list: Vec<Player>,
    player_img_url: Option<String>,
    player_name: Option<String>,
    player_score: Option<i32>,
    player_id: Option<i32>,
    success: bool,
    error: String,
}

impl Component for ChangeUser {
    type Message = ChangeUserMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let result = list_players().await;
            log!("{:?}", result);
            match result {
                Ok(res) => match res {
                    Ok(res) => ChangeUserMsg::PlayerList(res),
                    Err(err) => ChangeUserMsg::Error(err.to_string()),
                },
                Err(err) => ChangeUserMsg::Error(err.to_string()),
                }
            }
        );
        let username = ctx.props().username.clone();
        let password = ctx.props().password.clone();
        Self {
            username,
            password,
            user_list: Vec::new(),
            player_img_url: None,
            player_name: None,
            player_id: None,
            player_score: None,
            success: false,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChangeUserMsg::NewName(player_name) => {
                self.player_name = Some(player_name);
                true
            }
            ChangeUserMsg::NewImgUrl(player_img_url) => {
                self.player_img_url = Some(player_img_url);
                true
            }
            ChangeUserMsg::NewScore(player_score) => {
                self.player_score = Some(player_score);
                true
            }
            ChangeUserMsg::Success(res) => {
                if res.len() > 1 {
                    self.success = false;
                } else {
                    log!("{:?}", res);
                    self.success = true;
                }
                true
            }
            ChangeUserMsg::Error(err) => {
                self.error = err;
                true
            }
            ChangeUserMsg::Change => {
                use crate::api::change_player::change_player;
                let username = self.username.clone();
                let password = self.password.clone();
                let player_img_url = self.player_img_url.clone();
                let mut player_name = self.player_name.clone();
                if player_name.is_some() {
                    if player_name.clone().unwrap().len() < 2 {
                        player_name = None;
                    }
                }
                let player_id = self.player_id.unwrap().clone();
                let player_score = self.player_score.clone();
                ctx.link().send_future(async move {
                    let result = change_player(
                        username,
                        password,
                        player_img_url,
                        player_name,
                        player_score,
                        player_id,
                    ).await;
                    log!("{:?}", result);
                    match result {
                        Ok(res) => ChangeUserMsg::Success(res),
                        Err(err) => ChangeUserMsg::Error(err.to_string()),
                    }
                });
                false
            }
            ChangeUserMsg::PlayerList(players) => {
                self.user_list = players;
                true
            }
            ChangeUserMsg::NewId(id) => {
                self.player_id = Some(id);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.success {
            return html! {
                <h1>{"Udało się!"}</h1>
            };
        }
        if self.error.len() > 0 {
            return html! {
                <h1>{self.error.clone()}</h1>
            };
        }

        let change_user = ctx.link().callback(|_| ChangeUserMsg::Change);

        let new_name = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            ChangeUserMsg::NewName(value)
        });

        let new_img = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            ChangeUserMsg::NewImgUrl(value)
        });

        let new_score = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse()
                .unwrap();
            ChangeUserMsg::NewScore(value)
        });

        if self.player_id.is_none() {
            let choose_id = ctx
                .link()
                .callback(|e: MouseEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value().parse().unwrap();
                    log!("changing player with id: {:?}", value);

                    ChangeUserMsg::NewId(value)
                });
            return html! {
                <div class="change-user">
                    <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/admin_panel.css"/>
                    <h2>{"wybór gracza"}</h2>
                {
                    self.user_list.iter().map(|player| {
                        let (id, name) = (player.id, player.name.clone());
                        
                        html! {
                            <option onclick={choose_id.clone()} value={id.to_string()}>{name}</option>
                        }
                    }).collect::<Vec<VNode>>()
                }
                    </div>
            }
        }

        html! {
            <div class="change-user">
                <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/admin_panel.css"/>
                <h2>{"zmien użytkownika"}</h2>
                <input
                    type="text"
                    placeholder="nazwa gracza"
                    oninput={new_name}
                />
                <input
                    type="text"
                    placeholder="url obrazka"
                    oninput={new_img}
                />
                <input
                    type="number"
                    oninput={new_score}
                />
                <button class="change-user-button" onclick={change_user} disabled={self.player_img_url.is_none() || self.player_name.is_none()}>{"zmien"}</button>
            </div>
        }
    }
}

