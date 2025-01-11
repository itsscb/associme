use std::time::Duration;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form,
};
use chrono::Utc;
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::{db, Config};

use super::AccountAuth;

#[instrument(skip(config, auth, headers))]
// #[axum::debug_handler]
pub async fn login(
    headers: HeaderMap,
    State(config): State<Config>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    let ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("not found");
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("not found");

    let (id, role) = match db::account::login(&config.pool, &auth.email, &auth.password).await {
        Ok((id, role)) => (id, role),
        Err(e) => {
            if matches!(e, crate::errors::ApplicationError::Unauthorized) {
                warn!(email = &auth.email, error = ?e, ip = ip, user_agent = user_agent);
                return e.into_response();
            }
            error!(email = &auth.email, error = ?e, ip = ip, user_agent = user_agent);
            return e.into_response();
        }
    };

    info!(email = &auth.email, ip = ip, user_agent = user_agent);
    let expiration = Utc::now() + Duration::from_secs(60 * 15);
    let mut claims: paseto_maker::Claims = paseto_maker::Claims::new()
        .with_expiration(expiration.to_rfc3339())
        .with_subject("token");
    if let Err(err) = claims.set_claim("role", &role) {
        error!(error = ?err, ip = ip, user_agent = user_agent);
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let token = match config.token_maker.create_token(&claims) {
        Ok(token) => token,
        Err(err) => {
            error!(error = ?err, ip = ip, user_agent = user_agent);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let ref_expiration = Utc::now() + Duration::from_secs(60 * 60 * 24);
    let mut ref_claims: paseto_maker::Claims = paseto_maker::Claims::new()
        .with_expiration(ref_expiration.to_rfc3339())
        .with_subject("refresh_token");

    if let Err(err) = ref_claims.set_claim("role", &role) {
        error!(error = ?err, ip = ip, user_agent = user_agent);
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let refresh_token = match config.token_maker.create_token(&claims) {
        Ok(token) => token,
        Err(err) => {
            error!(error = ?err, ip = ip, user_agent = user_agent);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };
    info!(
        // expires_at = token.expires_at().to_rfc3339(),
        // account_id = token.id(),
        role = role.as_str(),
        ip = ip,
        user_agent = user_agent
    );

    if let Err(e) = db::session::new(
        &config.pool,
        &id,
        user_agent,
        ip,
        &refresh_token,
        &expiration,
        // refresh_token.expires_at(),
    )
    .await
    {
        error!(error = ?e, ip = ip, user_agent = user_agent);
        return e.into_response();
    };
    (
        StatusCode::OK,
        axum::response::Json(json!({"token": json!({"token": token, "role": role, "expires_at": expiration}), "refresh_token": json!({"token": refresh_token,"expires_at": ref_expiration})})),
    )
        .into_response()
}
