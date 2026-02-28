use crate::components::PostDetailInfo;
use crate::components::error::Error;
use crate::route::Route;
use crate::token_storage::TokenStorage;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Serialize)]
struct UpdatePostRequest {
    title: String,
    content: String,
}

#[derive(Properties, PartialEq)]
pub struct EditPostProps {
    pub id: String,
}

#[component(EditPost)]
pub fn edit_post(props: &EditPostProps) -> Html {
    let title = use_state(String::new);
    let content = use_state(String::new);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let navigator = use_navigator().unwrap();

    {
        let title = title.clone();
        let content = content.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();
        use_effect_with(id.clone(), move |id| {
            let title = title.clone();
            let content = content.clone();
            let loading = loading.clone();
            let error = error.clone();
            let id = id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:3000/api/posts/{}", id);
                let resp = Request::get(&url).send().await;
                match resp {
                    Ok(r) if r.status() == 404 => {
                        error.set(Some(format!("Post with id {} not found", id)));
                    }
                    Ok(r) => match r.json::<PostDetailInfo>().await {
                        Ok(data) => {
                            title.set(data.title);
                            content.set(data.content);
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
        let id = props.id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let title_val = (*title).clone();
            let content_val = (*content).clone();
            let saving = saving.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            let id = id.clone();

            saving.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let token = TokenStorage::get_token().unwrap_or_default();
                let update_data = UpdatePostRequest {
                    title: title_val,
                    content: content_val,
                };

                let url = format!("http://localhost:3000/api/posts/{}", id);
                let resp = Request::put(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&update_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        navigator.push(&Route::Home);
                    }
                    Ok(r) => match r.json::<Error>().await {
                        Ok(data) => {
                            error.set(Some(format!("Update failed: {}", data.message())));
                        }
                        Err(_) => {
                            error.set(Some(format!("Update failed with status: {}", r.status())));
                        }
                    },
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
            <h2>{"Edit Post"}</h2>
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
                                    value={(*title).clone()}
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
                                    value={(*content).clone()}
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
                                    {"Save Changes"}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            }
        </div>
    }
}
