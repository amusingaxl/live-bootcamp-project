use crate::domain::errors::InvalidEmailError;
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Email, InvalidEmailError> {
        if !email.validate_email() {
            return Err(InvalidEmailError);
        }

        Ok(Email(email.to_string()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_parse() {
        assert!(Email::parse("email").is_err());
        assert!(Email::parse("email@example").is_ok());
    }
}
