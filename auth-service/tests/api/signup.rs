use auth_service::routes::signup::SignupResponse;
use serde_json::json;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_201_for_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "Signup successful".to_string(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        json!({
            "password": "password123",
            "requires2FA": true
        }),
        json!({
                "email": random_email,
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
