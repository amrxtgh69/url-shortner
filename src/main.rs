use std::{net::{SocketAddr, TcpListener}, sync::Arc, time::Duration};
use axum::{
    Router,
    routing::{get, post},
};
use tokio;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    redis: RedisPool,
    base_url: String,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/shorten", post(create_short))
        .route("/:code", get(redirect))
        .with_state(app_state);    
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
