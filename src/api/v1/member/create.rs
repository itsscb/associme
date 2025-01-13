use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    db::{self, member::NewMember},
    Config,
};

#[instrument(skip(config))]
pub async fn create_member(
    State(config): State<Config>,
    Json(member): Json<NewMember>,
) -> impl IntoResponse {
    info!(member = ?member, "create member request");
    if let Some(member_id) = member.member_id {
        if member_id <= 0 {
            tracing::error!("member_id must not be negative");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "member_id must not be negative" })),
            )
                .into_response();
        }
    }
    match db::member::create(&config.pool, member).await {
        Ok(member) => {
            info!(member = ?member, "member created");
            (StatusCode::OK, Json(member.to_json())).into_response()
        }
        Err(e) => {
            tracing::error!(error = ?e, "failed to create member");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to create member"})),
            )
                .into_response()
        }
    }
}
