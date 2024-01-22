use crate::api::auth::Auth;
use crate::api::response::ServerResponse;
use crate::api::delete_user::delete_user;
use crate::api::delete_user::list_users;
use crate::router::AdminRoute;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use super::root_panel::RootPanel;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[derive(Debug)]
pub enum Stage {
    Nothing,
    Uploading,
    UploadedOk(ServerResponse),
    UploadedErr(String),
}

#[derive(Debug)]
pub enum Msg {
    SetUserList(Result<Vec<String>, reqwest::Error>),
    UsernameToDelete(String),
    Stage(Stage),
    Delete,
    Refresh,
    Nothing,
    BackToMainPanel,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: String,
    pub password: String,
}
#[derive(Debug)]
pub struct DeleteUser {
    username: String,
    password: String,
    username_to_delete: String,
    user_list: Vec<String>,
    stage: Stage,
    go_back: bool,
}

impl Component for DeleteUser {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if ctx.props().username.is_empty() && ctx.props().password.is_empty() {
            ctx.link().navigator().unwrap().push(&AdminRoute::RootPanel);
        }

        let username = ctx.props().username.clone();
        let password = ctx.props().password.clone();

        ctx.link().send_future(async move {
            let result = list_users(Auth { username, password }).await;
            match result {
                Ok(res) => Msg::SetUserList(Ok(res)),
                Err(err) => Msg::SetUserList(Err(err)),
            }
        });

        Self {
            stage: Stage::Nothing,
            username: ctx.props().username.clone(),
            password: ctx.props().password.clone(),
            user_list: Vec::default(),
            username_to_delete: String::new(),
            go_back: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Delete => {
                let password = self.password.clone();
                let username = self.username.clone();
                let username_to_delete = self.username_to_delete.clone();

                ctx.link().send_future(async move {
                    let result = delete_user(username, password, username_to_delete).await;
                    match result {
                        Ok(res) => Msg::Stage(Stage::UploadedOk(res)),
                        Err(err) => Msg::Stage(Stage::UploadedErr(err.to_string())),
                    }
                });

                self.stage = Stage::Uploading;

                true
            }
            Msg::SetUserList(user_list) => {
                let user_list = match user_list {
                    Ok(user_list) => user_list,
                    Err(_) => Vec::default(),
                };
                self.user_list = user_list;
                true
            }
            Msg::Stage(stage) => {
                self.stage = stage;

                true
            }
            Msg::Nothing => false,
            Msg::Refresh => {
                ctx.link()
                    .navigator()
                    .unwrap()
                    .push(&AdminRoute::AdminPanel);
                true
            }
            Msg::UsernameToDelete(username) => {
                self.username_to_delete = username;

                false
            }
            Msg::BackToMainPanel => {
                self.go_back = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.go_back {
            return html! {
                    <RootPanel username={self.username.clone()} password={self.password.clone()}/>
            };
        }
        match &self.stage {
            Stage::Nothing => {
                let oninput_username_to_delet = ctx.link().callback(|e: InputEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value();
                    Msg::UsernameToDelete(value)
                });

                let onclick = ctx.link().callback(|_| Msg::Delete);

                html! {
                <div>
                    <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/login.css"/>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::BackToMainPanel
                        })

                        }>{"Powrót do panelu głównego"}</button>
                        <div>
                            <label>{ "Użytkownik do usunięcia" }</label>
                            <input type="text" oninput={oninput_username_to_delet} onkeypress={
                            ctx.link().callback(move |e: KeyboardEvent| {
                                            if e.key() == "Enter" {
                                                Msg::Delete
                                            } else {
                                                Msg::Nothing
                                            }
                                        })
                                    }/><br/>
                            <h1>{"Lista użytkowników"}</h1>
                            <ul>
                                { for self.user_list.iter().map(|username| {
                                    log!("username: {}", username);
                                    html! {
                                        <li>{ username }</li>
                                    }
                                })}
                            </ul>
                            <button {onclick} >{"usuń"}</button>
                            </div>

                </div>

                }
            }
            Stage::Uploading => {
                html! {<h1>{"Oczekiwanie na odpowiedź serwera..."}</h1>}
            }
            Stage::UploadedOk(res) => match res {
                ServerResponse::Ok => {
                    html! {
                        <div>
                        <h1>{"Użytkownik usunięty"}</h1>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::Stage(Stage::Nothing)
                        })

                        }>{"Usuń kolejnego użytkownika"}</button>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::Refresh
                        })

                        }>{"odśwież stronę"}</button>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::BackToMainPanel
                        })

                        }>{"Powrót do panelu głównego"}</button>
                        </div>
                    }
                }
                _ => {
                    log!("{res}");
                    html! {<h1>{"wystąpił błąd przy usuwaniu użytkownika"}</h1>}
                }
            },
            Stage::UploadedErr(err) => {
                log!("{err}");
                html! {<h1>{"wystąpił błąd przy usuwaniu użytkownika"}</h1>}
            }
        }
    }
}
