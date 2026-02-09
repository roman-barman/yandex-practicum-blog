use crate::components::PostsList;
use yew::{Html, component, html};

#[component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container text-center">
            <div class="row">
                <div class="col">
                    <button type="button" class="btn btn-primary">{"Log In"}</button>
                </div>
                <div class="col">
                    <button type="button" class="btn btn-light">{"Sign In"}</button>
                </div>
                <div class="col-10">
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
