use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::{instrument, trace};

use crate::{api::extract_claims_from_request, db, Config};

#[instrument(skip(config, req))]
pub async fn get_account(State(config): State<Config>, req: Request) -> impl IntoResponse {
    match extract_claims_from_request(&req, &config) {
        Ok(claims) => {
            let Some(id) = claims.get_claim::<String>("id") else {
                return StatusCode::NOT_FOUND.into_response();
            };

            (db::account::get_account_by_id(&config.pool, &id).await).map_or_else(
                |_| StatusCode::NOT_FOUND.into_response(),
                |account| {
                    trace!(email = ?&account.email, "account found");
                    Json(json!(account)).into_response()
                },
            )
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
