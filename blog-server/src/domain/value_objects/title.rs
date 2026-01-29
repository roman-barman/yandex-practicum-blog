const TITLE_MAX_LENGTH: usize = 100;

#[derive(Debug, PartialEq)]
pub(crate) struct Title(String);

impl TryFrom<String> for Title {
    type Error = TitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TitleError::Empty);
        }
        if value.len() > TITLE_MAX_LENGTH {
            return Err(TitleError::TooLong);
        }
        Ok(Title(value))
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub(crate) enum TitleError {
    #[error("title is empty")]
    Empty,
    #[error("title is too long")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_title() {
        let title = Title::try_from("valid_title".to_string()).unwrap();
        assert_eq!(title.0, "valid_title");
    }

    #[test]
    fn create_invalid_title() {
        assert_eq!(Title::try_from("".to_string()), Err(TitleError::Empty));

        let long_title = "a".repeat(TITLE_MAX_LENGTH + 1);
        assert_eq!(Title::try_from(long_title), Err(TitleError::TooLong));
    }
}
