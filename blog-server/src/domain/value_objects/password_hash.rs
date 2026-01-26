#[derive(Debug, PartialEq)]
pub struct PasswordHash(String);

impl From<String> for PasswordHash {
    fn from(s: String) -> Self {
        Self(s)
    }
}
