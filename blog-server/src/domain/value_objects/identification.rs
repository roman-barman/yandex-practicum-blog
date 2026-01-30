use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub(crate) struct Identification(Uuid);

impl Identification {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for Identification {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl AsRef<Uuid> for Identification {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
