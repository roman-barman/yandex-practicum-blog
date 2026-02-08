use crate::components::*;
use yew::{Html, html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
