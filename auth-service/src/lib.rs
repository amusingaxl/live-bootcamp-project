use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve,
    serve::Serve,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::routes::{login, logout, signup, verify_2fa, verify_token};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status_code, body).into_response()
    }
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}
