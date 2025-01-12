use axum::{extract::Request, http::HeaderValue};
use paseto_maker::Claims;
use tracing::{error, warn};

use crate::Config;

pub mod middleware;
pub mod v1;

fn extract_claims_from_request(req: &Request, config: &Config) -> Result<Claims, String> {
    let auth = req.headers().get("Authorization");
    if let Some(auth) = auth {
        let token = extract_token(auth)?;
        match config.token_maker.verify_token(&token) {
            Ok(claims) => Ok(claims),
            Err(err) => {
                error!(error = ?err, "invalid token");
                Err("invalid token".to_string())
            }
        }
    } else {
        warn!("token not found");
        Err("token not found".to_string())
    }
}

fn extract_token(auth: &HeaderValue) -> Result<String, String> {
    match auth.to_str() {
        Ok(token) => {
            if !token.starts_with("Bearer ") {
                warn!("invalid token");
                return Err("invalid token".to_string());
            }
            Ok(token.trim_start_matches("Bearer ").to_string())
        }
        Err(err) => {
            error!(error = ?err, "failed to convert HeaderValue to str");
            Err("failed to convert HeaderValue to str".to_string())
        }
    }
}
