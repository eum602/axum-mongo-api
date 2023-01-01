mod api;
use api::health;
use dotenv::dotenv;
use std::{env, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post},
    BoxError, Router, Server,
};

use crate::api::orders;

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
        .route("/health", get(health::get))
        .route("/orders", get(orders::list).post(orders::create)) // handles gets and posts depenging of the method reaching the server
        .route("/orders/:id", get(orders::get))
        .route("/orders/:id/items", post(orders::add_item))
        .route("/orders/:id/items/:index", delete(orders::delete_item))
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
