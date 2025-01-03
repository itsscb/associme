use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::{Config, db};

#[derive(serde::Deserialize)]
pub struct ListSessionReq {
    pub account_id: uuid::Uuid,
}

#[instrument(skip(config, account_id))]
pub async fn list(
    State(config): State<Config>,
    account_id: Json<ListSessionReq>,
) -> impl IntoResponse {
    (db::session::list(&config.pool, &account_id.account_id).await).map_or_else(
        |e| {
            error!(account_id = &account_id.account_id.to_string(), error = ?e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response()
        },
        |sessions| {
            info!(
                account_id = &account_id.account_id.to_string(),
                sessions = sessions
                    .iter()
                    .map(|s| s.id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            (StatusCode::OK, axum::Json(sessions)).into_response()
        },
    )
}
