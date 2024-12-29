use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::db;

use super::AccountAuth;

#[instrument(skip(pool, auth))]
pub async fn login(
    State(pool): State<sqlx::PgPool>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::login(pool, &auth.email, &auth.password).await).map_or_else(
        |e| {
            if matches!(e, crate::errors::ApplicationError::Unauthorized) {
                warn!(email = &auth.email, error = ?e);
                (
                    StatusCode::UNAUTHORIZED,
                    axum::response::Json(json!({ "error": "Unauthorized" })),
                )
                    .into_response()
            } else {
                error!(email = &auth.email, error = ?e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::response::Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
        },
        |()| {
            info!(email = &auth.email);
            (
                StatusCode::OK,
                axum::response::Json(json!({"token": "login successful"})),
            )
                .into_response()
        },
    )
}
