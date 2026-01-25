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
    pub(crate) fn new(user_name: UserName, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id: Identification::new(),
            username: user_name,
            email,
            password_hash,
            created_at: DateTime::now(),
        }
    }
}
