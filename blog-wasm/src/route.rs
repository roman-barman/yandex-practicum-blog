use crate::components::*;
use yew::{Html, html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/post/:id")]
    PostDetail { id: String },
    #[at("/post/:id/edit")]
    EditPost { id: String },
    #[at("/post")]
    CreatePost,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::PostDetail { id } => html! { <PostDetail id={id} /> },
        Route::EditPost { id } => html! { <EditPost id={id} /> },
        Route::CreatePost => html! { <CreatePost /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
