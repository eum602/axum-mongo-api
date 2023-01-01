use axum::{extract::Path, http::StatusCode, Json};
use tracing::debug;
use uuid::Uuid;

use super::{request::AddItem, response::Order};

pub async fn create() -> (StatusCode, Json<Option<Order>>) {
    debug!("Creating a new order");
    (StatusCode::CREATED, Json(None))
}

pub async fn list() -> (StatusCode, Json<Option<Vec<Order>>>) {
    debug!("Listing all orders");
    (StatusCode::OK, Json(None))
}

pub async fn get(Path(id): Path<Uuid>) -> (StatusCode, Json<Option<Order>>) {
    debug!("Retrieving order with id: {id}");
    (StatusCode::OK, Json(None))
}

pub async fn add_item(Path(id): Path<Uuid>, Json(request): Json<AddItem>) -> StatusCode {
    debug!(
        "Adding item to order with id: {}, product Id: {} and quantity: {}",
        id, request.product_id, request.quantity
    );
    StatusCode::OK
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Deleting item from order with id: {id}, index: {index}");
    StatusCode::OK
}
