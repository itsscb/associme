use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::{instrument, trace};

use crate::{db, Config};

#[instrument(skip(config))]
// TODO: Make fields selectable via QueryParams and return only some fields as default
pub async fn list_members(State(config): State<Config>) -> impl IntoResponse {
    db::member::list(&config.pool)
        .await
        .map(|members| {
            trace!("members listed");
            Json(json!({ "members": members }))
        })
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to list members");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
