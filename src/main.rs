use dotenv::dotenv;
use std::{env, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    BoxError, Router, Server,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().expect("Set your configuration in an .env file");

    let message = "Define a SERVER=host:port pair in your .env file";
    let server_address = env::var("SERVER").expect(&message);
    let server_address = server_address.parse().expect(&message);
    info!("server_address: http://{:?}/", server_address);
    Server::bind(&server_address);
    let app = Router::new()
        .route("/", get(|| async { "Welcome to main page" }))
        .route("/greetings", get(greet))
        .route("/health", get(health))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .fallback(fallback_handler);

    Server::bind(&server_address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn greet() -> &'static str {
    // tokio::time::sleep(Duration::from_secs(8)).await;
    "Hello world!"
}

/// shutdown handler
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect ctrl - ctrl shutdown");
    println!("Signal shutting down");
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route found for uri: {}", uri);
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}

/// health check
#[tracing::instrument]
async fn health() -> StatusCode {
    info!("new incoming health check status request");
    StatusCode::OK
}
