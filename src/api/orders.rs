use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use tracing::debug;
use uuid::Uuid;

use crate::order_store::OrderStoreNewType;

use super::{request::AddItem, response::Order};

const USER_ID: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);

type State = Arc<OrderStoreNewType>;

#[axum_macros::debug_handler] // adding this debugger just to exemplify debugging
pub async fn create(Extension(state): Extension<State>) -> (StatusCode, Json<Option<Order>>) {
    debug!("Creating a new order");
    if let Ok(order) = state.0.create_order(USER_ID).await {
        // state.0 -> getting the element '0' from the 'Box'
        (StatusCode::OK, Json(Some(Order::from(order))))
    } else {
        (StatusCode::FORBIDDEN, Json(None))
    }
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
