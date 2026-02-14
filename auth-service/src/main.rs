use auth_service::{
    Application,
    app_state::AppState,
    services::user_store::{HashMapUserStore, make_user_store}
};

#[tokio::main]
async fn main() {
    let user_store = make_user_store(HashMapUserStore::default());
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
