mod api;
mod in_mem_order_store;
mod order_store;
use api::health;
use dotenv::dotenv;
use in_mem_order_store::InMemOrderStore;
use std::{env, sync::Arc, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post},
    BoxError, Extension, Router, Server,
};

use crate::{api::orders, order_store::OrderStoreNewType};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().expect("Set your configuration in an .env file");

    // repository
    let repo = InMemOrderStore::new();

    let state = Arc::new(OrderStoreNewType::new(repo)); // allowing repo to be avalable in muliple threads
                                                        // 'Arc' to allow many copies
                                                        // OrderNewType -> just the type we defined
                                                        // All this stuff just to use this in an abstract way rather that in a specific one.

    let message = "Define a SERVER=host:port pair in your .env file";
    let server_address = env::var("SERVER").expect(&message);
    let server_address = server_address.parse().expect(&message);
    info!("server_address: http://{:?}/", server_address);
    Server::bind(&server_address);

    let order_routes = Router::new()
        .route("/", get(orders::list).post(orders::create)) // handles gets and posts depenging of the method reaching the server
        .route("/:id", get(orders::get))
        .route("/:id/items", post(orders::add_item))
        .route("/:id/items/:index", delete(orders::delete_item))
        .layer(Extension(state)); // Axum stores this in a dictionary key value where the key is the "type" of what is being stored in it.
    let app = Router::new()
        .route("/health", get(health::get))
        .nest("/orders", order_routes)
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
