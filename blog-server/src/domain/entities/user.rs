use crate::domain::value_objects::{DateTime, Email, Identification, PasswordHash, UserName};

#[derive(Debug, PartialEq)]
pub(crate) struct User {
    id: Identification,
    username: UserName,
    email: Email,
    password_hash: PasswordHash,
    created_at: DateTime,
}

impl User {
    pub(crate) fn new(username: UserName, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id: Identification::new(),
            username,
            email,
            password_hash,
            created_at: DateTime::now(),
        }
    }

    pub(crate) fn restore(
        id: Identification,
        username: UserName,
        email: Email,
        password_hash: PasswordHash,
        created_at: DateTime,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
            created_at,
        }
    }

    pub(crate) fn id(&self) -> &Identification {
        &self.id
    }

    pub(crate) fn username(&self) -> &UserName {
        &self.username
    }

    pub(crate) fn email(&self) -> &Email {
        &self.email
    }

    pub(crate) fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub(crate) fn created_at(&self) -> &DateTime {
        &self.created_at
    }
}
