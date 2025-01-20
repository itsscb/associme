use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    db::{self, member::NewMember},
    errors::ApplicationError,
    Config,
};

#[instrument(skip(config, member))]
pub async fn create_member(
    State(config): State<Config>,
    Extension(account_id): Extension<uuid::Uuid>,
    Json(mut member): Json<NewMember>,
) -> impl IntoResponse {
    info!("create member request");
    member.created_by = Some(account_id);
    // return (StatusCode::OK, Json(json!(member))).into_response();
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
            info!(member_id = ?member.id, "member created");
            (StatusCode::OK, Json(member.to_json())).into_response()
        }
        Err(e) => {
            if matches!(e, ApplicationError::Duplicate) {
                tracing::warn!("duplicate member");
                return (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "e-mail address already taken" })),
                )
                    .into_response();
            }
            tracing::error!(error = ?e, "failed to create member");
            e.into_response()
            // (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     Json(json!({"error": "failed to create member"})),
            // )
            //     .into_response()
        }
    }
}
