use crate::components::root::delete_user::DeleteUser;
use crate::components::root::change_password::ChangePassword;
use crate::components::root::add_user::AddUser;
use crate::components::login::Login;
use yew::prelude::*;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[derive(Debug)]
pub enum Msg {
    Password(String),
    Username(String),
    LoggedIn(bool),
    AddUser,
    ChangePassword,
    DeleteUser,
}

#[derive(Debug, PartialEq)]
pub enum SelectedOption {
    Nothing,
    AddUser,
    ChangePassword,
    DeleteUser,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct RootPanel {
    logged_in: bool,
    username: String,
    password: String,
    stage: SelectedOption,
}

impl Component for RootPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            logged_in: !ctx.props().username.is_empty() && !ctx.props().password.is_empty(),
            username: ctx.props().username.clone(),
            password: ctx.props().password.clone(),
            stage: SelectedOption::Nothing,
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
            Msg::AddUser => {
                self.stage = SelectedOption::AddUser;
                true
            }
            Msg::ChangePassword => {
                self.stage = SelectedOption::ChangePassword;
                true
            }
            Msg::DeleteUser => {
                self.stage = SelectedOption::DeleteUser;
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

        let change_password = ctx.link().callback(|_| Msg::ChangePassword);
        let new_user = ctx.link().callback(|_| Msg::AddUser);
        let delete_user = ctx.link().callback(|_| Msg::DeleteUser);
        if self.logged_in {
            match self.stage {
                SelectedOption::Nothing => html! {
                    <div class="admin-panel">
                        <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/admin_panel.css"/>
                        <h2 style="color: black;">{"wybierz opcję"}</h2>

                        <button class="option add-post" onclick={new_user} >{"Dodaj użytkownika"}</button>
                        <button class="option remove-post" onclick={delete_user}>{"Usuń użytkownika"}</button>
                        <button class="option change-post" onclick={change_password}>{"Zmień hasło"}</button>
                        </div>
               },
                SelectedOption::AddUser => html! {
                    <AddUser username={self.username.clone()} password={self.password.clone()} />
                },
                SelectedOption::ChangePassword => html! {
                    <ChangePassword username={self.username.clone()} password={self.password.clone()} />
                },
                SelectedOption::DeleteUser => html! {
                    <DeleteUser username={self.username.clone()} password={self.password.clone()} />
                },

            }
        } else {
            html! {
                <div>
                    <link rel="stylesheet" type="text/css" href="http://localhost:8080/css/login.css"/>
                        <Login password={on_password} username={on_username} logged_in={on_login}/>
                </div>
            }
        }
    }
}
