const USERNAME_MIN_LENGTH: usize = 5;
const USERNAME_MAX_LENGTH: usize = 20;

#[derive(Debug, PartialEq)]
pub(crate) struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = UserNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(UserNameError::Empty);
        }
        if value.len() < USERNAME_MIN_LENGTH {
            return Err(UserNameError::TooShort);
        }
        if value.len() > USERNAME_MAX_LENGTH {
            return Err(UserNameError::TooLong);
        }
        Ok(UserName(value))
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum UserNameError {
    #[error("username is empty")]
    Empty,
    #[error("username is too short")]
    TooShort,
    #[error("username is too long")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_username() {
        let username = UserName::try_from("valid_username".to_string()).unwrap();
        assert_eq!(username.0, "valid_username");
    }

    #[test]
    fn create_invalid_username() {
        assert_eq!(
            UserName::try_from("".to_string()),
            Err(UserNameError::Empty)
        );
        assert_eq!(
            UserName::try_from("a".to_string()),
            Err(UserNameError::TooShort)
        );
        assert_eq!(
            UserName::try_from("aaaaaaaaaaaaaaaaaaaaaa".to_string()),
            Err(UserNameError::TooLong)
        );
    }
}
