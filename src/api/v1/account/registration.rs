use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::{Config, db};

use super::AccountAuth;

#[instrument(skip(config, auth))]
pub async fn registration(
    State(config): State<Config>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::create_account(config.pool, &auth.email, &auth.password).await).map_or_else(
        |e| {
            error!(email = &auth.email, error = ?e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response()
        },
        |account| {
            info!(email = &auth.email);
            (StatusCode::CREATED, account.to_json()).into_response()
        },
    )
}
