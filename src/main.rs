use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router, Server,
};

#[tokio::main]
async fn main() {
    let server_address = ([127, 0, 0, 1], 8080).into();
    println!("server_address: http://{:?}/", server_address);
    Server::bind(&server_address);
    let app = Router::new()
        .route("/", get(|| async { "Welcome to main page" }))
        .route("/greetings", get(greet))
        .fallback(fallback_handler);

    Server::bind(&server_address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn greet() -> &'static str {
    "Hello world!"
}

/// shutdown handler
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect ctrl - ctrl shutdown");
    println!("Signal shutting down");
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}
