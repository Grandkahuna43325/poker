mod app;
mod api;
mod components;
mod router;
mod state;

fn main() {
    state::State::init("http://localhost:8080".to_string());
    yew::Renderer::<app::App>::new().render();
}
