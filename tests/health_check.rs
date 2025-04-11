use axum::serve::Listener;
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

    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {}

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
