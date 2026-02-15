use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStoreError},
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = User::new(
        Email::parse(&request.email).map_err(map_invalid_input_error)?,
        Password::parse(&request.password).map_err(map_invalid_input_error)?,
        request.requires_2fa,
    );
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

fn map_invalid_input_error<E>(_err: E) -> AuthAPIError {
    AuthAPIError::InvalidCredentials
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SignupResponse {
    pub message: String,
}
