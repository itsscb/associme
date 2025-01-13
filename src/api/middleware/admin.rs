use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tracing::{error, instrument, trace, warn};

use crate::{api::extract_claims_from_request, Config};

#[instrument(skip(config, req, next))]
pub async fn admin(State(config): State<Config>, req: Request, next: Next) -> impl IntoResponse {
    match extract_claims_from_request(&req, &config) {
        Ok(claims) => {
            if let Some(role) = claims.get_claim::<String>("role") {
                if role.to_lowercase() != "admin" {
                    warn!("not an admin");
                    return (StatusCode::UNAUTHORIZED, "only for admins").into_response();
                }
            } else {
                warn!("no role found in token");
                return (StatusCode::UNAUTHORIZED, "no role in token").into_response();
            }
            trace!("role verified");
            next.run(req).await
        }
        Err(err) => {
            error!(error = ?err, "failed to verify role");
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
