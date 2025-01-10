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
    let mut claims: paseto_maker::Claims =
        paseto_maker::Claims::new().with_expiration(expiration.to_rfc3339());
    if let Err(err) = claims.set_claim("role", &role) {
        error!(error = ?err, ip = ip, user_agent = user_agent);
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
    // let mut claims = ClaimsS::new();
    // if let Err(err) = claims.add("role", &role) {
    //     return err.into_response();
    // }

    // let maker = paseto_maker::Maker::new(&config.keypair.secret).unwrap();
    // let (token, refresh_token) = match TokenS::new_pair(
    //     id.to_string(),
    //     claims,
    //     Duration::from_secs(60 * 15),
    //     Duration::from_secs(60 * 60 * 24),
    //     &config.keypair.secret,
    // ) {
    //     Ok((token, refresh_token)) => (token, refresh_token),
    //     Err(e) => {
    //         error!(error = ?e, ip = ip, user_agent = user_agent);
    //         return (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             axum::response::Json(json!({ "error": "Internal server error" })),
    //         )
    //             .into_response();
    //     }
    // };

    let token = &config.token_maker.create_token(&claims).unwrap();
    let refresh_token = &config.token_maker.create_token(&claims).unwrap();
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
        refresh_token,
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
        axum::response::Json(json!({"token": token, "refresh_token": refresh_token})),
    )
        .into_response()
}
