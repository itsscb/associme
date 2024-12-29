use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::db;

use super::AccountAuth;

#[instrument(skip(pool))]
pub async fn registration(
    State(pool): State<sqlx::PgPool>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::create_account(pool, &auth.email, &auth.password).await).map_or_else(
        |e| {
            error!("Failed to create account: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response()
        },
        |account| {
            info!("Created account with email: {}", auth.email);
            (StatusCode::CREATED, account.to_json()).into_response()
        },
    )
}
