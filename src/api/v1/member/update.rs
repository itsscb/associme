use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    db::{self, member::UpdateMember},
    errors::ApplicationError,
    Config,
};

#[instrument(skip(config))]
#[axum::debug_handler]
pub async fn update_member(
    State(config): State<Config>,
    Extension(account_id): Extension<uuid::Uuid>,
    Json(mut member): Json<UpdateMember>,
) -> impl IntoResponse {
    member.changed_by = account_id;
    if db::account::get_account_by_id(&config.pool, &member.changed_by.to_string())
        .await
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "unauthorized"})),
        )
            .into_response();
    }

    match db::member::update(&config.pool, member).await {
        Ok(member) => {
            info!(member = ?member, "member updated");
            (StatusCode::OK, Json(member.to_json())).into_response()
        }
        Err(e) => {
            tracing::error!(error = ?e, "failed to update member");
            match e {
                ApplicationError::NotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "member not found"})),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "failed to update member"})),
                )
                    .into_response(),
            }
        }
    }
}
