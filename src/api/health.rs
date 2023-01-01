use axum::http::StatusCode;
use tracing::info;

/// health check
#[tracing::instrument]
pub async fn get() -> StatusCode {
    info!("new incoming health check status request");
    StatusCode::OK
}
