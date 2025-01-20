use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::{instrument, trace};

use crate::{db, errors::ApplicationError, Config};

#[instrument(skip(config))]
pub async fn get_member(
    State(config): State<Config>,
    Path(member_id): Path<String>,
) -> impl IntoResponse {
    db::member::get_by_id(&config.pool, &member_id)
        .await
        .map(|member| {
            trace!(member = member_id.as_str(), "member found");
            Json(json!({"member": member}))
        })
        .map_err(|e| {
            if matches!(e, ApplicationError::NotFound) {
                tracing::info!(member = member_id.as_str(), "member not found");
                StatusCode::NOT_FOUND
            } else {
                tracing::error!(error = ?e, "failed to get member");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}
