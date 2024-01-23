use crate::api::auth::check_credentials;
use crate::api::response::ServerResponse;
use crate::api::auth::Auth;
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Submit(FetchState),
    Fetch,
    Nothing,
    NewUsername(String),
    NewPassword(String),
}

#[derive(Debug)]
pub enum FetchState {
    Fetching,
    Success(ServerResponse),
    Failed(String),
    NotFetching,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub username: Callback<String>,
    pub password: Callback<String>,
    pub logged_in: Callback<bool>,
}

pub struct Login {
    pub fetch_state: FetchState,
    pub username: String,
    pub password: String,
    pub logged_in: bool,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            fetch_state: FetchState::NotFetching,
            username: String::new(),
            password: String::new(),
            logged_in: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let handle_username = self.username.clone();
        let handle_password = self.password.clone();
        match msg {
            Msg::Submit(fetch_state) => {
                match &fetch_state {
                    FetchState::Fetching => {}
                    FetchState::Success(i) => match i {
                        ServerResponse::Ok => {
                            self.logged_in = true;
                        }
                        _ => {
                            self.logged_in = false;
                        }
                    },
                    FetchState::Failed(err) => {
                        self.logged_in = false;
                        log!("{}", err);
                    }
                    FetchState::NotFetching => {}
                }
                self.fetch_state = fetch_state;
                true
            }
            Msg::Fetch => {
                ctx.link().send_future(async move {
                    let result = check_credentials(Auth {
                        username: handle_username,
                        password: handle_password,
                    })
                    .await;
                    match result {
                        Ok(res) => Msg::Submit(FetchState::Success(res)),
                        Err(err) => Msg::Submit(FetchState::Failed(err.to_string())),
                    }
                });
                ctx.link().send_message(Msg::Submit(FetchState::Fetching));
                true
            }
            Msg::NewUsername(username) => {
                self.username = username;
                false
            }
            Msg::NewPassword(password) => {
                self.password = password;
                false
            }
            Msg::Nothing => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(move |_| Msg::Fetch);

        let oninput_username = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            Msg::NewUsername(value)
        });
        let oninput_password = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            Msg::NewPassword(value)
        });

        match &self.fetch_state {
            FetchState::Fetching => {
                html! {
                    <div>
                        <h1>{"Logowanie..."}</h1>
                    </div>
                }
            }
            FetchState::Success(res) => match res {
                ServerResponse::Ok => {
                    ctx.props().logged_in.emit(true);
                    ctx.props().username.emit(String::from(&self.username));
                    ctx.props().password.emit(String::from(&self.password));
                    html!{}
                }
                ServerResponse::BadPassword => {
                    let onkeypress = ctx.link().callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" {
                            Msg::Fetch
                        } else {
                            Msg::Nothing
                        }
                    });
                    html! {
                    <div id="main">
                    <link rel="stylesheet" type="text/css" href="https://d9fd-188-146-95-12.ngrok-free.app/css/login.css"/>
                      <div id="login-input-div">
                        <h2 style="color: black">{"Błędne hasło lub nazwa użytkownika"}</h2>
                        <input
                          type="text"
                          placeholder="nazwa użytkownika"
                          oninput={oninput_username}
                          class="login-input"
                        /><br />
                        <input
                          type="password"
                          oninput={oninput_password}
                          placeholder="hasło"
                          class="login-input"
                          {onkeypress}
                        />

                        <div>
                          <button {onclick} id="login-button">{"Submit"}</button>
                        </div>
                      </div>
                    </div>
                                }
                }
                _ => {
                    log!("{}", res.to_string());
                    html! {
                    <div>
                        <h1>{"Błąd w trakcie logowania"}</h1>
                    </div>
                    }
                }
            },
            FetchState::Failed(i) => {
                log!("{}", i);
                html! {
                    <div>
                        <h1>{"coś się zjebało z serwerem :)"}</h1>
                    </div>
                }
            }
            FetchState::NotFetching => {
                html! {
                <div id="main">
                    <div id="login-input-div">
                        <h2 style="color: black">{"Zaloguj"}</h2>
                         <input type="text" placeholder="nazwa użytkownika" oninput={oninput_username} class="login-input"/><br/>
                         <input type="password" oninput={oninput_password} placeholder="hasło" class="login-input" onkeypress={
                            ctx.link().callback(move |e: KeyboardEvent| {
                                            if e.key() == "Enter" {
                                                Msg::Fetch
                                            } else {
                                                Msg::Nothing
                                            }
                                        })
                                    } />

                        <div>
                            <button {onclick} id="login-button">{"Submit"}</button>
                        </div>
                    </div>
                </div>
                }
            }
        }
    }
}
