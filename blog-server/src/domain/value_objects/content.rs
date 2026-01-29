#[derive(Debug, PartialEq)]
pub(crate) struct Content(String);

impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}
