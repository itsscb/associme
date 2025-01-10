use axum::{extract::State, response::IntoResponse};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde_json::json;
use tracing::instrument;

use crate::Config;

#[instrument(skip(config))]
pub async fn public_key(State(config): State<Config>) -> impl IntoResponse {
    let public_key_bytes = config.token_maker.public_key_as_bytes();
    let public_key = BASE64_STANDARD.encode(public_key_bytes);
    // let public_key = base64::encode(public_key_bytes);
    axum::response::Json(json!({"public_key": public_key})).into_response()
}
