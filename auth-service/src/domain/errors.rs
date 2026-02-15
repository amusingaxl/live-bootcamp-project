use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid email")]
pub struct InvalidEmailError;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid password")]
pub struct InvalidPasswordError;
