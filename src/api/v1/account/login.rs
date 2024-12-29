use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::db;

use super::AccountAuth;

#[instrument(skip(pool))]
pub async fn login(
    State(pool): State<sqlx::PgPool>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::login(pool, &auth.email, &auth.password).await).map_or_else(
        |e| {
            if matches!(e, crate::errors::ApplicationError::Unauthorized) {
                warn!("Unauthorized login by: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    axum::response::Json(json!({ "error": "Unauthorized" })),
                )
                    .into_response()
            } else {
                error!("Login failed for '{}': {:?}", &auth.email, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::response::Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
        },
        |()| {
            info!("Login with email: {}", auth.email);
            (StatusCode::OK, "login successful").into_response()
        },
    )
}
