use auth_service::Application;
use reqwest::{Client, Response};
use uuid::Uuid;
use serde::Serialize;

pub struct TestApp {
    pub address: String,
    pub http_client: Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("0.0.0.0:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address);

        let _ = tokio::spawn(app.run());

        let http_client = Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> Response where Body: Serialize {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login(&self) -> Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_2fa(&self) -> Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_token(&self) -> Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
