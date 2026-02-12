use crate::route::Route;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{Link, use_navigator};
use crate::components::error::Error;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let login_data = LoginRequest {
                    username: username_val,
                    password: password_val,
                };

                let resp = Request::post("http://localhost:3000/api/auth/login")
                    .json(&login_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => match r.json::<LoginResponse>().await {
                        Ok(data) => {
                            if let Err(e) = LocalStorage::set("token", data.token) {
                                error.set(Some(format!("Failed to save token: {}", e)));
                            } else {
                                navigator.push(&Route::Home);
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse response: {}", e)));
                        }
                    },
                    Ok(r) if r.status() == 401 => {
                        error.set(Some("Invalid username or password".to_string()));
                    }
                    Ok(r) => {
                        match r.json::<Error>().await {
                            Ok(data) => {
                                error.set(Some(format!("Login failed: {}", data.message())));
                            }
                            Err(_) => {
                                error.set(Some(format!("Login failed with status: {}", r.status())));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="container mt-5">
            <div class="row justify-content-center">
                <div class="col-md-4">
                    <div class="card">
                        <div class="card-header text-center">
                            <h3>{"Login"}</h3>
                        </div>
                        <div class="card-body">
                            if let Some(err) = &*error {
                                <div class="alert alert-danger" role="alert">
                                    {err.clone()}
                                </div>
                            }
                            <form onsubmit={on_submit}>
                                <div class="mb-3">
                                    <label for="username" class="form-label">{"Username"}</label>
                                    <input
                                        type="text"
                                        class="form-control"
                                        id="username"
                                        value={(*username).clone()}
                                        oninput={on_username_input}
                                        required=true
                                    />
                                </div>
                                <div class="mb-3">
                                    <label for="password" class="form-label">{"Password"}</label>
                                    <input
                                        type="password"
                                        class="form-control"
                                        id="password"
                                        value={(*password).clone()}
                                        oninput={on_password_input}
                                        required=true
                                    />
                                </div>
                                <div class="d-grid gap-2">
                                    <button
                                        type="submit"
                                        class="btn btn-primary"
                                        disabled={*loading}
                                    >
                                        if *loading {
                                            <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                        }
                                        {"Login"}
                                    </button>
                                </div>
                            </form>
                        </div>
                        <div class="card-footer text-center">
                            <p class="mb-0">{"Don't have an account? "} <Link<Route> to={Route::Register}>{"Sign Up"}</Link<Route>></p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
