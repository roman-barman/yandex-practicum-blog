use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Error {
    error: String,
}

impl Error {
    pub(super) fn message(&self) -> &str {
        &self.error
    }
}
