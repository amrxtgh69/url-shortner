use tokio;
use std::{net::{SocketAddr, TcpListener}, sync::Arc, time::Duration};
use axum::{
    Router,
    routing::{get, post},
};

#[derive(Clone)]
struct AppState {
    db: PgPool,
    redis: RedisPool,
    base_url: String,
}


#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState { db, redis, base_url });
    let app = Router::new()
        .route("/shorten", post(create_short))
        .route("/:code", get(redirect))
        .with_state(app_state);    
    
    println!("Server listening on port 3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
