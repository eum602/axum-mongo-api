use axum::{extract::Path, http::StatusCode};
use tracing::debug;
use uuid::Uuid;

pub async fn create() -> StatusCode {
    debug!("Creating a new orders");
    StatusCode::CREATED
}

pub async fn list() -> StatusCode {
    debug!("Listing all orders");
    StatusCode::OK
}

pub async fn get(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Retrieving order with id: {id}");
    StatusCode::OK
}

pub async fn add_item(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Adding item to order with id: {id}");
    StatusCode::OK
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Deleting item from order with id: {id}, index: {index}");
    StatusCode::OK
}
