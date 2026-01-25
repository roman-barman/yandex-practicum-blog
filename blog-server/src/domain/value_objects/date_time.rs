#[derive(Debug, PartialEq)]
pub(crate) struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub(crate) fn now() -> Self {
        Self(chrono::Utc::now())
    }
}
