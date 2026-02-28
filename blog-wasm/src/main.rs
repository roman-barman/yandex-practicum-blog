pub mod components;
mod route;
mod token_storage;

use crate::route::{Route, switch};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
