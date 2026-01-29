use crate::domain::value_objects::{Content, DateTime, Identification, Title};

#[derive(Debug, PartialEq)]
pub(crate) struct Post {
    id: Identification,
    title: Title,
    content: Content,
    author_id: Identification,
    created_at: DateTime,
    updated_at: DateTime,
}

impl Post {
    pub(crate) fn new(title: Title, content: Content, author_id: Identification) -> Self {
        Self {
            id: Identification::new(),
            title,
            content,
            author_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub(crate) fn id(&self) -> &Identification {
        &self.id
    }

    pub(crate) fn content(&self) -> &Content {
        &self.content
    }

    pub(crate) fn title(&self) -> &Title {
        &self.title
    }

    pub(crate) fn author_id(&self) -> &Identification {
        &self.author_id
    }

    pub(crate) fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub(crate) fn updated_at(&self) -> &DateTime {
        &self.updated_at
    }
}
