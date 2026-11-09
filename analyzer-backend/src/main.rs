use analys_backend::handle_analys_full;
use axum::{Router, routing::post};
#[tokio::main]
async fn main() {
    // our router
    let app = Router::new().route("/", post(handle_analys_full));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
