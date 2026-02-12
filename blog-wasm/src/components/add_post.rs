use crate::components::PostDetailInfo;
use crate::route::Route;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::error::Error;

#[derive(Serialize)]
struct CreatePostRequest {
    title: String,
    content: String,
}

#[component(CreatePost)]
pub fn create_post() -> Html {
    let title = use_state(String::new);
    let content = use_state(String::new);
    let loading = use_state(|| false);
    let saving = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let navigator = use_navigator().unwrap();

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let content = content.clone();
        let saving = saving.clone();
        let error = error.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let title_val = (*title).clone();
            let content_val = (*content).clone();
            let saving = saving.clone();
            let error = error.clone();
            let navigator = navigator.clone();

            saving.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let token = LocalStorage::get::<String>("token").unwrap_or_default();
                let update_data = CreatePostRequest {
                    title: title_val,
                    content: content_val,
                };

                let url = format!("http://localhost:3000/api/posts");
                let resp = Request::post(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&update_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        navigator.push(&Route::Home);
                    }
                    Ok(r) => {
                        match r.json::<Error>().await {
                            Ok(data) => {
                                error.set(Some(format!("Create failed: {}", data.message())));
                            }
                            Err(_) => {
                                error.set(Some(format!("Create failed with status: {}", r.status())));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                    }
                }
                saving.set(false);
            });
        })
    };

    html! {
        <div class="container mt-4">
            <h2>{"Create Post"}</h2>
            if *loading {
                <div class="d-flex justify-content-center">
                    <div class="spinner-border" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            } else {
                <div class="card">
                    <div class="card-body">
                        if let Some(err) = &*error {
                            <div class="alert alert-danger" role="alert">
                                {err.clone()}
                            </div>
                        }
                        <form onsubmit={on_submit}>
                            <div class="mb-3">
                                <label for="title" class="form-label">{"Title"}</label>
                                <input
                                    type="text"
                                    class="form-control"
                                    id="title"
                                    oninput={on_title_input}
                                    required=true
                                />
                            </div>
                            <div class="mb-3">
                                <label for="content" class="form-label">{"Content"}</label>
                                <textarea
                                    class="form-control"
                                    id="content"
                                    rows="5"
                                    oninput={on_content_input}
                                    required=true
                                />
                            </div>
                            <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                                <button
                                    type="button"
                                    class="btn btn-secondary me-md-2"
                                    onclick={
                                        let navigator = navigator.clone();
                                        Callback::from(move |_| navigator.push(&Route::Home))
                                    }
                                >
                                    {"Cancel"}
                                </button>
                                <button
                                    type="submit"
                                    class="btn btn-primary"
                                    disabled={*saving}
                                >
                                    if *saving {
                                        <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                    }
                                    {"Save"}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            }
        </div>
    }
}
