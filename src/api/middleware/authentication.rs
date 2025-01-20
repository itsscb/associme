use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tracing::{error, instrument, trace, warn};
use uuid::Uuid;

use crate::{api::extract_claims_from_request, Config};

#[instrument(skip(config, req, next))]
pub async fn authentication(
    State(config): State<Config>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    match extract_claims_from_request(&req, &config) {
        Ok(claims) => {
            if let Some(expiration) = claims.get_expiration() {
                if expiration < chrono::Utc::now() {
                    warn!("token expired");
                    return (StatusCode::UNAUTHORIZED, "token expired").into_response();
                }
            } else {
                warn!("token: no expiration set");
                return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
            }
            trace!("token verified");

            if let Some(id) = claims.get_claim::<Uuid>("id") {
                req.extensions_mut().insert(id);
            } else {
                error!("failed to get account_id from token");
                return StatusCode::UNAUTHORIZED.into_response();
            }
            next.run(req).await
        }
        Err(err) => {
            error!(error = ?err, "failed to verify token");
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
    // if let Some(auth) = req.headers().get("Authorization") {
    //     match auth.to_str() {
    //         Ok(token) => {
    //             if !token.starts_with("Bearer ") {
    //                 warn!("invalid token");
    //                 return StatusCode::UNAUTHORIZED.into_response();
    //             }
    //             let token = token.trim_start_matches("Bearer ");

    //             if let Ok(claims) = &config.token_maker.verify_token(token) {
    //                 if let Some(expiration) = claims.get_expiration() {
    //                     if expiration < chrono::Utc::now() {
    //                         warn!("token expired");
    //                         return (StatusCode::UNAUTHORIZED, "token expired").into_response();
    //                     }
    //                 } else {
    //                     warn!("token: no expiration set");
    //                     return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    //                 }
    //                 trace!("token verified");
    //                 next.run(req).await
    //             } else {
    //                 warn!("invalid token");
    //                 (StatusCode::UNAUTHORIZED, "invalid token").into_response()
    //             }
    //         }
    //         Err(err) => {
    //             error!( error = ?err, "failed to convert HeaderValue to str");
    //             StatusCode::UNAUTHORIZED.into_response()
    //         }
    //     }
    // } else {
    //     warn!("token not found");
    //     StatusCode::UNAUTHORIZED.into_response()
    // }
}
