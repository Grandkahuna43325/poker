use std::sync::{Mutex, OnceLock};

pub struct State {
    pub api_url: String,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

impl State {
    pub fn init(api_url: String) {
        STATE.get_or_init(|| Mutex::new(State { api_url }));
    }

    pub fn _get() -> impl std::ops::Deref<Target = State> {
        STATE.get().expect("State not initialized").lock().unwrap()
    }

    pub fn get_url() -> String {
        STATE.get().expect("State not initialized").lock().unwrap().api_url.clone()
    }

    pub fn _update_api_url(new_url: String) {
        STATE.get().as_ref().unwrap().lock().unwrap().api_url = new_url;
    }
}
