use crate::components::error::Error;
use crate::route::Route;
use crate::token_storage::TokenStorage;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Debug, serde::Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Clone, PartialEq, Debug, serde::Deserialize)]
struct PostsResponse {
    posts: Vec<Post>,
    total: usize,
    limit: usize,
    offset: usize,
}

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    #[prop_or_default]
    pub refresh_version: usize,
}

#[component(PostsList)]
pub fn posts_list(props: &PostsListProps) -> Html {
    let posts = use_state(Vec::<Post>::new);
    let total = use_state(|| 0usize);
    let limit = use_state(|| 5usize);
    let offset = use_state(|| 0usize);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let is_logged_in = use_state(TokenStorage::is_logged_in);
    let refresh_trigger = use_state(|| 0);
    let parent_refresh = props.refresh_version;

    {
        let posts = posts.clone();
        let total = total.clone();
        let limit = limit.clone();
        let offset = offset.clone();
        let loading = loading.clone();
        let error = error.clone();
        let refresh_trigger = refresh_trigger.clone();
        let is_logged_in = is_logged_in.clone();
        use_effect_with(
            (*limit, *offset, *refresh_trigger, parent_refresh),
            move |(limit, offset, _, _)| {
                let l = *limit;
                let o = *offset;
                let posts = posts.clone();
                let total = total.clone();
                let loading = loading.clone();
                let error = error.clone();
                loading.set(true);
                error.set(None);
                is_logged_in.set(TokenStorage::is_logged_in());
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://localhost:3000/api/posts?limit={}&offset={}", l, o);
                    let resp = Request::get(&url).send().await;
                    match resp {
                        Ok(r) => match r.json::<PostsResponse>().await {
                            Ok(data) => {
                                posts.set(data.posts);
                                total.set(data.total);
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
            },
        );
    }

    let on_prev = {
        let offset = offset.clone();
        let limit = limit.clone();
        Callback::from(move |_| {
            let new_offset = offset.saturating_sub(*limit);
            offset.set(new_offset);
        })
    };

    let on_next = {
        let offset = offset.clone();
        let limit = limit.clone();
        let total = total.clone();
        Callback::from(move |_| {
            let next = *offset + *limit;
            if next < *total {
                offset.set(next);
            }
        })
    };

    let on_delete = {
        let refresh_trigger = refresh_trigger.clone();
        let error = error.clone();
        let loading = loading.clone();
        Callback::from(move |id: String| {
            let refresh_trigger = refresh_trigger.clone();
            let error = error.clone();
            let loading = loading.clone();
            let token = TokenStorage::get_token().unwrap_or_default();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let url = format!("http://localhost:3000/api/posts/{}", id);
                let resp = Request::delete(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        refresh_trigger.set(*refresh_trigger + 1);
                    }
                    Ok(r) => match r.json::<Error>().await {
                        Ok(data) => {
                            error.set(Some(format!("Delete failed: {}", data.message())));
                        }
                        Err(_) => {
                            error.set(Some(format!("Delete failed with status: {}", r.status())));
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    let current_page = if *limit == 0 {
        1
    } else {
        (*offset / *limit) + 1
    };
    let total_pages = if *limit == 0 {
        1
    } else {
        (*total).div_ceil(*limit).max(1)
    };

    html! {
        <div class="container mt-4">
            <h2>{"Posts"}</h2>

            if let Some(err) = &*error {
                <div class="alert alert-danger" role="alert">{err.clone()}</div>
            }

            <div class="d-flex justify-content-between align-items-center mb-2">
                <div class="btn-group" role="group" aria-label="Pagination">
                    <button type="button" class="btn btn-outline-secondary" onclick={on_prev.clone()} disabled={*loading || *offset == 0}>{"Previous"}</button>
                    <button type="button" class="btn btn-outline-secondary" onclick={on_next.clone()} disabled={*loading || (*offset + *limit) >= *total}>{"Next"}</button>
                </div>
                <div class="text-muted small">
                    {format!("Page {} of {} ({} items)", current_page, total_pages, *total)}
                </div>
            </div>

            <table class="table table-striped table-hover">
                <thead class="table-dark">
                    <tr>
                        <th scope="col">{"#"}</th>
                        <th scope="col">{"Title"}</th>
                        <th scope="col">{"Content"}</th>
                        <th scope="col">{"Actions"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        if *loading {
                            html! {<tr><td colspan="4"><div class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></div>{" Loading..."}</td></tr>}
                        } else if posts.is_empty() {
                            html! {<tr><td colspan="4" class="text-center text-muted">{"No posts"}</td></tr>}
                        } else {
                            posts.iter().cloned().map(|post| {
                                html! {
                                    <tr key={post.id.clone()}>
                                        <th scope="row">{post.id.clone()}</th>
                                        <td>{post.title}</td>
                                        <td>{post.content}</td>
                                        <td>
                                            <Link<Route> to={Route::PostDetail { id: post.id.clone() }} classes="btn btn-sm btn-primary me-2">
                                                {"View"}
                                            </Link<Route>>
                                            if *is_logged_in {
                                                <Link<Route> to={Route::EditPost { id: post.id.clone() }} classes="btn btn-sm btn-outline-warning me-2">
                                                    {"Update"}
                                                </Link<Route>>
                                                <button
                                                    type="button"
                                                    class="btn btn-sm btn-outline-danger"
                                                    onclick={
                                                        let on_delete = on_delete.clone();
                                                        let id = post.id.clone();
                                                        Callback::from(move |_| on_delete.emit(id.clone()))
                                                    }
                                                    disabled={*loading}
                                                >
                                                    {"Delete"}
                                                </button>
                                            }
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    }
                </tbody>
            </table>
        </div>
    }
}
