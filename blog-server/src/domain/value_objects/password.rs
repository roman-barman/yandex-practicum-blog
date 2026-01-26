use secrecy::{ExposeSecret, SecretString};

const MIN_PASSWORD_LENGTH: usize = 8;
const SPECIAL_CHARACTERS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];

#[derive(Debug)]
pub(crate) struct Password(SecretString);

impl AsRef<[u8]> for Password {
    fn as_ref(&self) -> &[u8] {
        self.0.expose_secret().as_bytes()
    }
}

impl TryFrom<SecretString> for Password {
    type Error = PasswordError;

    fn try_from(value: SecretString) -> Result<Self, Self::Error> {
        let password = value.expose_secret();

        if password.is_empty() {
            return Err(PasswordError::Empty);
        }

        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(PasswordError::TooShort);
        }

        let mut has_upper_case = false;
        let mut has_lower_case = false;
        let mut has_digit = false;
        let mut has_special_char = false;

        for ch in password.chars() {
            if ch.is_uppercase() {
                has_upper_case = true;
            } else if ch.is_lowercase() {
                has_lower_case = true;
            } else if ch.is_ascii_digit() {
                has_digit = true;
            } else if SPECIAL_CHARACTERS.contains(&ch) {
                has_special_char = true;
            }
        }

        if !has_upper_case || !has_lower_case || !has_digit || !has_special_char {
            return Err(PasswordError::Invalid);
        }

        Ok(Password(value))
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum PasswordError {
    #[error("password is empty")]
    Empty,
    #[error("password is too short")]
    TooShort,
    #[error(
        "password must contain at least one uppercase letter, one lowercase letter, one digit and one special character"
    )]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = Password::try_from(SecretString::from("Abc123!&")).unwrap();
        assert_eq!(password.0.expose_secret(), "Abc123!&");
    }

    #[test]
    fn test_invalid_password() {
        let err = Password::try_from(SecretString::from("")).unwrap_err();
        assert_eq!(err, PasswordError::Empty);

        let err = Password::try_from(SecretString::from("abc")).unwrap_err();
        assert_eq!(err, PasswordError::TooShort);

        let err = Password::try_from(SecretString::from("Abc12345")).unwrap_err();
        assert_eq!(err, PasswordError::Invalid);

        let err = Password::try_from(SecretString::from("abc12345!")).unwrap_err();
        assert_eq!(err, PasswordError::Invalid);

        let err = Password::try_from(SecretString::from("Abc!!!!!!!")).unwrap_err();
        assert_eq!(err, PasswordError::Invalid);
    }
}
