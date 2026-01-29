use crate::domain::value_objects::{Content, DateTime, Identification, Title};

#[derive(Debug, PartialEq)]
pub(crate) struct Blog {
    id: Identification,
    title: Title,
    content: Content,
    author_id: Identification,
    created_at: DateTime,
    updated_at: DateTime,
}
