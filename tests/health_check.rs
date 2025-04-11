use std::collections::HashMap;

use axum_leptos_zero2prod::FormData;
use reqwest::{Client, StatusCode};
use tokio::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = &listener.local_addr().unwrap().port();
    let server = axum_leptos_zero2prod::run(listener);
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_return_200_for_a_valid_data() {
    let app = spawn_app().await;
    let client = Client::new();
    let form = FormData {
        name: "karol".to_string(),
        email: "karol@test.com".to_string(),
    };

    let response = client
        .post(format!("{}/subscriptions", app))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(StatusCode::CREATED, response.status())
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = Client::new();

    let form = FormData {
        name: "karol".to_string(),
        email: "karol@test.com".to_string(),
    };

    let mut invalid_forms = HashMap::new();
    let mut one = form.clone();
    let mut two = form.clone();
    let mut three = form.clone();
    one.email = "".to_owned();
    two.name = "".to_owned();
    three.email = "".to_owned();
    three.name = "".to_owned();
    invalid_forms.insert("missing the email", one);
    invalid_forms.insert("missing the name", two);
    invalid_forms.insert("missing both name and email", three);

    for (error_message, invalid_form) in invalid_forms {
        let response = client
            .post(format!("{}/subscriptions", app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&invalid_form)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not faile with 400 Bad Request when the payload was {}",
            error_message
        )
    }
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    dbg!(address.clone());
    let client = Client::new();
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to bind");

    assert!(response.status().is_success());
    assert_eq!(response.status(), StatusCode::OK);
}
