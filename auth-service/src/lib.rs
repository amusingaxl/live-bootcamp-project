use axum::{Router, routing::post, serve, serve::Serve};
use crate::routes::{signup, login, logout, verify_2fa, verify_token};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod routes;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            ;

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
