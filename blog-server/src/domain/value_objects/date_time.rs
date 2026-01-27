#[derive(Debug, PartialEq)]
pub(crate) struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub(crate) fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for DateTime {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}
