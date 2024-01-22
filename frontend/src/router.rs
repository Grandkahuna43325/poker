use crate::components::root::root_panel::RootPanel;
use crate::components::game::game::Game as GameComponent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::admin::AdminPanel;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    ListPoints,
    #[at("/game")]
    Game,
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin")]
    AdminPanel,
    #[at("/admin/root")]
    RootPanel,
    #[not_found]
    #[at("/admin/404")]
    NotFound,
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::ListPoints));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::ListPoints => html! {"dee"},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Game => html! { <GameComponent /> },
        Route::AdminRoot | Route::Admin => html! { <Switch<AdminRoute> render={switch_admin} /> },
    }
}

fn switch_admin(route: AdminRoute) -> Html {
    match route {
        AdminRoute::AdminPanel => {
            html! {<AdminPanel/>}
        }
        AdminRoute::RootPanel => {
            html! {<RootPanel username="" password=""/>}
        },
        AdminRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
