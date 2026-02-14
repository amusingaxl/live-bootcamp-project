use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User, data_stores::UserStoreError},
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    request.validate()?;

    let user = User::new(&request.email, &request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store
        .add_user(user)
        .await
        .map_err(map_user_store_error)?;

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "Signup successful".to_string(),
        }),
    ))
}

fn map_user_store_error(err: UserStoreError) -> AuthAPIError {
    match err {
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        UserStoreError::InvalidCredentials => AuthAPIError::InvalidCredentials,
        // User not found does not make sense in the signup flow, but it's required here so the pattern matching is exhaustive
        UserStoreError::UserNotFound => AuthAPIError::UnexpectedError,
        UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

const MIN_PASSWORD_LENGTH: usize = 8;

impl SignupRequest {
    pub fn validate(&self) -> Result<(), AuthAPIError> {
        if !self.email.contains('@') || self.password.len() < MIN_PASSWORD_LENGTH {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SignupResponse {
    pub message: String,
}
