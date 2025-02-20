use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{instrument, trace};

use crate::{db, errors::ApplicationError, Config};

#[instrument(skip(config))]
pub async fn delete_member(
    State(config): State<Config>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    db::member::delete_by_id(&config.pool, &id)
        .await
        .map(|member| {
            trace!(member = id.as_str(), "member found");
            Json(member.to_json())
        })
        .map_err(|e| {
            if matches!(e, ApplicationError::NotFound) {
                tracing::info!(member = id.as_str(), "member not found");
                StatusCode::NOT_FOUND
            } else {
                tracing::error!(error = ?e, "failed to get member");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}
