use crate::{api::add_player::add_player, router::AdminRoute};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

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
                    <h1>{"zmiana posta"}</h1>
            },
            Msg::RemoveUser => html! {
                <h1>{"usuwanie posta"}</h1>
            },
            Msg::ChooseAction => html! {
                <div class="admin-panel">
                    <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/admin_panel.css"/>
                    <h2 style="color: black;">{"wybierz opcję"}</h2>

                    <button class="option add-post" onclick={add_post} >{"dodaj post"}</button>
                    <button class="option remove-post" onclick={remove_post}>{"usuń post"}</button>
                    <button class="option change-post" onclick={change_post}>{"zmień post"}</button>
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
