mod app;
mod components;
mod router;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
