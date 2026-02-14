use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(&request.email, &request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "Signup successful".to_string(),
    });

    (StatusCode::CREATED, response)
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
