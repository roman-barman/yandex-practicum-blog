pub mod components;
mod route;

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
    yew::Renderer::<App>::new().render();
}
