use axum::{Router, extract::Path, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    dbg!(&name);
    format!("Hello {name}")
}

async fn greets() -> String {
    "hello from inside".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greets))
        .route("/greet/{name}", get(greet));
    let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
