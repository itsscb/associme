use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::{Config, db};

#[derive(serde::Deserialize)]
pub struct RevokeTokenReq {
    pub token: String,
}

#[instrument(skip(config, token))]
pub async fn revoke(
    State(config): State<Config>,
    token: Json<RevokeTokenReq>,
) -> impl IntoResponse {
    (db::session::revoke(&config.pool, &token.token).await).map_or_else(
        |e| {
            error!(token = &token.token, error = ?e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response()
        },
        |()| {
            info!(token = &token.token, "Session blocked");
            (StatusCode::OK).into_response()
        },
    )
}
