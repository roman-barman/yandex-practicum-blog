use uuid::Uuid;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Identification(Uuid);

impl Identification {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
