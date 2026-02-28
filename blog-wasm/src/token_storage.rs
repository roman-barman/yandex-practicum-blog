use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "token";

pub struct TokenStorage;

impl TokenStorage {
    pub fn is_logged_in() -> bool {
        LocalStorage::get::<String>("token").is_ok()
    }

    pub fn get_token() -> Option<String> {
        LocalStorage::get(TOKEN_KEY).ok()
    }

    pub fn set_token(token: String) -> gloo_storage::Result<()> {
        LocalStorage::set(TOKEN_KEY, token)
    }

    pub fn clear() {
        LocalStorage::delete(TOKEN_KEY);
    }
}
