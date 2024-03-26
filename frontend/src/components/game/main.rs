use crate::components::game::game::Game;
use crate::components::login::Login;
use yew::prelude::*;


#[derive(Debug)]
pub enum Msg {
    Password(String),
    Username(String),
    LoggedIn(bool),
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {}

#[derive(Debug)]
pub struct AdminPanel {
    logged_in: bool,
    username: String,
    password: String,
}

impl Component for AdminPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            logged_in: false,
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Password(password) => {
                self.password = password;
                false
            }
            Msg::Username(username) => {
                self.username = username;
                false
            }
            Msg::LoggedIn(logged_in) => {
                self.logged_in = logged_in;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_username = ctx
            .link()
            .callback(|username: String| Msg::Username(username));
        let on_login = ctx
            .link()
            .callback(|logged_in: bool| Msg::LoggedIn(logged_in));

        let on_password = ctx
            .link()
            .callback(|password: String| Msg::Password(password));

        html! {
            <div>
                // <link rel="stylesheet" type="text/css" href="https://poker.kfkorulczyk.pl/css/login.css"/>
                <link rel="stylesheet" type="text/css" href="https://poker.kfkorulczyk.pl/css/login.css"/>
                if self.logged_in {
                    <Game username={self.username.clone()} password={self.password.clone()}/>
                }else{
                    <Login password={on_password} username={on_username} logged_in={on_login}/>
                }
            </div>
        }
    }
}
