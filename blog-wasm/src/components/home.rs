use crate::components::PostsList;
use crate::route::Route;
use gloo_storage::{LocalStorage, Storage};
use yew::{Callback, Html, component, html, use_state};
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

#[component(Home)]
pub fn home() -> Html {
    let is_logged_in = use_state(|| LocalStorage::get::<String>("token").is_ok());
    let refresh_version = use_state(|| 0);

    let on_logout = {
        let is_logged_in = is_logged_in.clone();
        let refresh_version = refresh_version.clone();
        Callback::from(move |_| {
            LocalStorage::delete("token");
            is_logged_in.set(false);
            refresh_version.set(*refresh_version + 1);
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
                if !*is_logged_in {
                    <Link<Route> to={Route::Register} classes="btn btn-light">{"Sign Up"}</Link<Route>>
                } else {
                    <Link<Route> to={Route::CreatePost} classes="btn btn-primary">{"Create Post"}</Link<Route>>
                }
            </div>
            <div class="col">
            </div>
        </div>
        <div class="row">
            <div class="col">
                <PostsList refresh_version={*refresh_version} />
            </div>
        </div>
    </div>
    }
}
