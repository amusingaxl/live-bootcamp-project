use auth_service::{Application, app_state::AppState, services::HashMapUserStore};
use reqwest::{Client, Response};
use serde::Serialize;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashMapUserStore::default();
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, "0.0.0.0:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address);

        std::mem::drop(tokio::spawn(app.run()));

        let http_client = Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login(&self) -> Response {
        self.http_client
            .post(format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_2fa(&self) -> Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_token(&self) -> Response {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
