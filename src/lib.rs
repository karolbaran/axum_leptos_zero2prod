use axum::{
    Form, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
async fn subscribe(Form(data): Form<FormData>) -> impl IntoResponse {
    dbg!(data);
    StatusCode::CREATED
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe));
    axum::serve(listener, app).await.unwrap();
}
