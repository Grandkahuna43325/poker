use crate::api::response::ServerResponse;
use crate::api::add_user::add_user;
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
    NewUsername(String),
    NewPassword(String),
    NewPassword2(String),
    Stage(Stage),
    Change,
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
pub struct AddUser {
    username: String,
    password: String,
    new_username: String,
    new_password: String,
    matching_password: bool,
    stage: Stage,
    go_back: bool,
}

impl Component for AddUser {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if ctx.props().username.is_empty() && ctx.props().password.is_empty() {
            ctx.link().navigator().unwrap().push(&AdminRoute::RootPanel);
        }

        Self {
            stage: Stage::Nothing,
            username: ctx.props().username.clone(),
            password: ctx.props().password.clone(),
            new_username: String::new(),
            new_password: String::new(),
            matching_password: false,
            go_back: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewPassword(passwd) => {
                self.new_password = passwd;

                false
            }
            Msg::NewPassword2(passwd) => {
                self.matching_password = passwd == self.new_password;

                true
            }
            Msg::Change => {
                let password = self.password.clone();
                let username = self.username.clone();
                let new_password = self.new_password.clone();
                let new_username = self.new_username.clone();

                ctx.link().send_future(async move {
                    let result = add_user(username, password, new_username, new_password).await;
                    match result {
                        Ok(res) => Msg::Stage(Stage::UploadedOk(res)),
                        Err(err) => Msg::Stage(Stage::UploadedErr(err.to_string())),
                    }
                });

                self.stage = Stage::Uploading;

                true
            }
            Msg::Stage(stage) => {
                self.stage = stage;

                true
            }
            Msg::Nothing => false,
            Msg::Refresh => {
                ctx.link().navigator().unwrap().push(&AdminRoute::RootPanel);
                true
            }
            Msg::NewUsername(username) => {
                self.new_username = username;

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
                let oninput_new_password = ctx.link().callback(|e: InputEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value();
                    Msg::NewPassword(value)
                });

                let oninput_new_password2 = ctx.link().callback(|e: InputEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value();
                    Msg::NewPassword2(value)
                });

                let oninput_new_username = ctx.link().callback(|e: InputEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value();
                    Msg::NewUsername(value)
                });

                let onclick = ctx.link().callback(|_| Msg::Change);

                html! {
                <div>
                    // <link rel="stylesheet" type="text/css" href="https://poker.kfkorulczyk.pl/css/login.css"/>
                    <link rel="stylesheet" type="text/css" href="https://poker.kfkorulczyk.pl/css/login.css"/>
                        <div>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::BackToMainPanel
                        })

                        }>{"Powrót do panelu głównego"}</button>
                            <label>{ "Nazwa użytkownika:" }</label>
                            <input type="text" oninput={oninput_new_username} /><br/>
                            <label>{ "Password:" }</label>
                            <input type="password" oninput={oninput_new_password} /><br/>
                            <label>{ "Confirm Password:" }</label>
                            <input type="password" oninput={oninput_new_password2} onkeypress={
                            ctx.link().callback(move |e: KeyboardEvent| {
                                            if e.key() == "Enter" {
                                                Msg::Change
                                            } else {
                                                Msg::Nothing
                                            }
                                        })
                                    }/><br/>
                            if !self.matching_password {
                                <h1>{"hasła są różne"}</h1>
                            }
                            <button disabled={!self.matching_password} {onclick} >{"Dodaj użytkownika"}</button>
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
                        <h1>{"Użytkownik dodany"}</h1>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::Stage(Stage::Nothing)
                        })

                        }>{"Dodaj następnego"}</button>
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
                    html! {
                       <div>
                           <button onclick={ctx.link().callback(|_| {
                               Msg::Refresh
                           })

                       }>{"odśwież stronę"}</button>

                       <h1>{"wystąpił błąd przy dodawaniu użytkownika"}</h1>
                       </div>
                            }
                }
            },
            Stage::UploadedErr(err) => {
                log!("{err}");
                html! {<h1>{"wystąpił błąd przy dodawaniu użytkownika"}</h1>}
            }
        }
    }
}
