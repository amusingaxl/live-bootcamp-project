use crate::domain::errors::InvalidPasswordError;

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Password, InvalidPasswordError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(InvalidPasswordError);
        }

        Ok(Password(password.to_string()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse() {
        assert!(Password::parse("pwd").is_err());
        assert!(Password::parse("p@ssword").is_ok());
    }
}
