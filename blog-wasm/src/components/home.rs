use yew::{Html, component, html};

#[component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Hello, world!"}</h1>
        </div>
    }
}
