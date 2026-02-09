use crate::route::Route;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[component(Register)]
pub fn register() -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let success = use_state(|| false);
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
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
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let success = success.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let success = success.clone();
            let loading = loading.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let register_data = RegisterRequest {
                    username: username_val,
                    email: email_val,
                    password: password_val,
                };

                let resp = Request::post("http://localhost:3000/api/auth/register")
                    .json(&register_data)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        success.set(true);
                    }
                    Ok(r) => {
                        error.set(Some(format!(
                            "Registration failed with status: {}",
                            r.status()
                        )));
                    }
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    if *success {
        let navigator = navigator.clone();
        return html! {
            <div class="container mt-5">
                <div class="row justify-content-center">
                    <div class="col-md-4">
                        <div class="alert alert-success" role="alert">
                            <h4 class="alert-heading">{"Registration Successful!"}</h4>
                            <p>{"Your account has been created. You can now log in."}</p>
                            <hr />
                            <button
                                class="btn btn-primary"
                                onclick={Callback::from(move |_| navigator.push(&Route::Login))}
                            >
                                {"Go to Login"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        };
    }

    html! {
        <div class="container mt-5">
            <div class="row justify-content-center">
                <div class="col-md-4">
                    <div class="card">
                        <div class="card-header text-center">
                            <h3>{"Register"}</h3>
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
                                    <label for="email" class="form-label">{"Email"}</label>
                                    <input
                                        type="email"
                                        class="form-control"
                                        id="email"
                                        value={(*email).clone()}
                                        oninput={on_email_input}
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
                                        {"Register"}
                                    </button>
                                </div>
                            </form>
                        </div>
                        <div class="card-footer text-center">
                            <p class="mb-0">{"Already have an account? "} <button class="btn btn-link p-0 pb-1" onclick={Callback::from(move |_| navigator.push(&Route::Login))}>{"Log In"}</button></p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
