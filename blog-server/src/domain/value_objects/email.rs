use regex::Regex;
use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<Result<Regex, regex::Error>> = OnceLock::new();
const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";

#[derive(Debug, PartialEq)]
pub(crate) struct Email(String);

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        };
        let email_regex = EMAIL_REGEX.get_or_init(|| Regex::new(EMAIL_PATTERN));
        let email_regex = match email_regex {
            Ok(regex) => regex,
            Err(err) => return Err(EmailError::Regex(err.clone())),
        };
        if !email_regex.is_match(&value) {
            return Err(EmailError::Invalid);
        }

        Ok(Email(value))
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum EmailError {
    #[error("email is empty")]
    Empty,
    #[error("regex is invalid")]
    Regex(regex::Error),
    #[error("email is invalid")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_email() {
        let email = Email::try_from("test@gmail.com".to_string()).unwrap();
        assert_eq!(email.0, "test@gmail.com");
    }

    #[test]
    fn create_invalid_email() {
        assert_eq!(Email::try_from("".to_string()), Err(EmailError::Empty));
        assert_eq!(
            Email::try_from("test".to_string()),
            Err(EmailError::Invalid)
        );
    }
}
