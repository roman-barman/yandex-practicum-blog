use gloo_net::http::Request;
use yew::prelude::*;
use crate::route::Route;
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Debug, serde::Deserialize)]
pub struct PostDetailInfo {
    pub id: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
    pub id: String,
}

#[component(PostDetail)]
pub fn post_detail(props: &PostDetailProps) -> Html {
    let post = use_state(|| Option::<PostDetailInfo>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let post = post.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();
        use_effect_with(id.clone(), move |id| {
            let post = post.clone();
            let loading = loading.clone();
            let error = error.clone();
            let id = id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:3000/api/posts/{}", id);
                let resp = Request::get(&url).send().await;
                match resp {
                    Ok(r) => match r.json::<PostDetailInfo>().await {
                        Ok(data) => {
                            post.set(Some(data));
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse response: {}", e)));
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container mt-4">
            <Link<Route> to={Route::Home} classes="btn btn-outline-primary mb-3">{"Back to Home"}</Link<Route>>
            
            if *loading {
                <div class="d-flex justify-content-center">
                    <div class="spinner-border" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            } else if let Some(err) = &*error {
                <div class="alert alert-danger" role="alert">{err.clone()}</div>
            } else if let Some(p) = &*post {
                <div class="card">
                    <div class="card-header">
                        <h2>{p.title.clone()}</h2>
                    </div>
                    <div class="card-body">
                        <p class="card-text" style="white-space: pre-wrap;">{&p.content}</p>
                    </div>
                    <div class="card-footer text-muted">
                        <div>{format!("Author ID: {}", p.user_id)}</div>
                        <div>{format!("Created: {}", p.created_at)}</div>
                        <div>{format!("Updated: {}", p.updated_at)}</div>
                    </div>
                </div>
            }
        </div>
    }
}
