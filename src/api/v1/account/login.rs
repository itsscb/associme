use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::{Config, db, models::Token};

use super::AccountAuth;

#[instrument(skip(config, auth))]
pub async fn login(
    State(config): State<Config>,
    // State(keypair): State<AsymmetricKeyPair<V4>>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::login(config.pool, &auth.email, &auth.password).await).map_or_else(
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
        |(id, role)| {
            info!(email = &auth.email);
            match Token::new(id, role, &config.keypair.secret) {
                Ok(token) => {
                    info!(
                        token = token.token.as_str(),
                        expires_at = token.expires_at.as_str(),
                        account_id = token.id.as_str(),
                        role = token.role.as_str()
                    );
                    (StatusCode::OK, axum::response::Json(token)).into_response()
                }
                Err(e) => {
                    error!(error = ?e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        axum::response::Json(json!({ "error": "Internal server error" })),
                    )
                        .into_response()
                }
            }
            // .map_err(|e| {
            //     error!(error = ?e,"Could not generate token");
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         axum::response::Json(json!({ "error": "Internal server error" })),
            //     )
            //         .into_response()
            // })?;

            // info!(
            //     token = token.token.as_str(),
            //     expires_at = token.expires_at.as_str(),
            //     account_id = token.id.as_str(),
            //     role = token.role.as_str()
            // );
            // (StatusCode::OK, axum::response::Json(token)).into_response()
        },
    )
}
