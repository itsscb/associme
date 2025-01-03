use std::time::Duration;

use axum::{
    Form,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::{
    Config, db,
    models::token::{Claims, Token},
};

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
    let mut claims = Claims::new();
    if let Err(err) = claims.add("role", &role) {
        error!(error = ?err, ip = ip, user_agent = user_agent);
        return err.into_response();
    }

    let (token, refresh_token) = match Token::new_pair(
        id.to_string(),
        claims,
        Duration::from_secs(60 * 15),
        Duration::from_secs(60 * 60 * 24),
        &config.keypair.secret,
    ) {
        Ok((token, refresh_token)) => (token, refresh_token),
        Err(e) => {
            error!(error = ?e, ip = ip, user_agent = user_agent);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response();
        }
    };

    info!(
        expires_at = token.expires_at().to_rfc3339(),
        account_id = token.id(),
        role = role.as_str(),
        ip = ip,
        user_agent = user_agent
    );

    if let Err(e) = db::session::new(
        &config.pool,
        &id,
        user_agent,
        ip,
        refresh_token.token(),
        refresh_token.expires_at(),
    )
    .await
    {
        error!(error = ?e, ip = ip, user_agent = user_agent);
        return e.into_response();
    };
    (
        StatusCode::OK,
        axum::response::Json(json!({"token": token, "refresh_token": refresh_token})),
    )
        .into_response()
}
