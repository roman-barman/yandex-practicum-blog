use crate::components::PostsList;
use crate::route::Route;
use yew_router::prelude::Link;
use yew::{Html, component, html, use_state, use_effect_with, Callback};
use gloo_storage::{LocalStorage, Storage};

#[component(Home)]
pub fn home() -> Html {
    let is_logged_in = use_state(|| LocalStorage::get::<String>("token").is_ok());

    {
        let is_logged_in = is_logged_in.clone();
        use_effect_with((), move |_| {
            is_logged_in.set(LocalStorage::get::<String>("token").is_ok());
            || ()
        });
    }

    let on_logout = {
        let is_logged_in = is_logged_in.clone();
        Callback::from(move |_| {
            LocalStorage::delete("token");
            is_logged_in.set(false);
        })
    };

    html! {
        <div class="container text-center">
            <div class="row mt-3 mb-3">
                <div class="col-auto">
                    if *is_logged_in {
                        <button type="button" class="btn btn-outline-danger" onclick={on_logout}>{"Log Out"}</button>
                    } else {
                        <Link<Route> to={Route::Login} classes="btn btn-primary">{"Log In"}</Link<Route>>
                    }
                </div>
                <div class="col-auto">
                    <Link<Route> to={Route::Register} classes="btn btn-light">{"Sign Up"}</Link<Route>>
                </div>
                <div class="col">
                </div>
            </div>
            <div class="row">
                <div class="col">
                    <PostsList />
                </div>
            </div>
        </div>
        }
}
