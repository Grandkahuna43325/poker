use crate::components::root::root_panel::RootPanel;
use crate::api::response::ServerResponse;
use crate::api::change_password::change_password;
use crate::router::AdminRoute;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

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
pub struct ChangePassword {
    username: String,
    password: String,
    username_to_change: String,
    new_password: String,
    matching_password: bool,
    stage: Stage,
    go_back: bool,
}

impl Component for ChangePassword {
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
            username_to_change: String::new(),
            new_password: String::new(),
            matching_password: false,
            go_back: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewUsername(username) => {
                self.username_to_change = username;
                false
            }
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
                let username_to_change = self.username_to_change.clone();
                let new_password = self.new_password.clone();

                ctx.link().send_future(async move {
                    let result =
                        change_password(username, password, username_to_change, new_password).await;
                    match result {
                        Ok(res) => Msg::Stage(Stage::UploadedOk(res)),
                        Err(err) => Msg::Stage(Stage::UploadedErr(err.to_string())),
                    }
                });

                self.stage = Stage::Uploading;

                self.password = self.new_password.clone();

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
                let oninput_new_username = ctx.link().callback(|e: InputEvent| {
                    let target = e.target().unwrap();
                    let value = target.unchecked_into::<HtmlInputElement>().value();
                    Msg::NewUsername(value)
                });

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

                let onclick = ctx.link().callback(|_| Msg::Change);

                html! {
                <div>
                    // <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/login.css"/>
                    <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/login.css"/>
                        <div>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::BackToMainPanel
                        })

                        }>{"Powrót do panelu głównego"}</button>
                            <label>{ "Nazwa użytkownika:" }</label>
                            <input type="text" oninput={oninput_new_username} /><br/>
                            <label>{ "Hasło:" }</label>
                            <input type="password" oninput={oninput_new_password} /><br/>
                            <label>{ "Powtórz hasło:" }</label>
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
                            <button disabled={!self.matching_password} {onclick} >{"zmień hasło"}</button>
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
                        <h1>{"Hasło zmienione!"}</h1>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::Stage(Stage::Nothing)
                        })

                        }>{"Zmień kolejne hasło"}</button>

                        <button onclick={ctx.link().callback(|_| {
                            Msg::Refresh
                        })}> {"Odśwież stronę"} </button>
                        <button onclick={ctx.link().callback(|_| {
                            Msg::BackToMainPanel
                        })
                        }>{"Wróć do panelu głównego"}</button>
                        </div>
                    }
                }
                _ => {
                    log!("{res}");
                    html! {<h1>{"wystąpił błąd przy zmienianiu hasła"}</h1>}
                }
            },
            Stage::UploadedErr(err) => {
                log!("{err}");
                html! {<h1>{"błąd przy zmienianiu hasła..."}</h1>}
            }
        }
    }
}
