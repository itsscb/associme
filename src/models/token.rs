use std::time::Duration;

use pasetors::{claims::Claims, keys::AsymmetricSecretKey, public, version4::V4};
use serde::{Deserialize, Serialize};

use crate::errors::ApplicationError;

const TOKEN_LIFETIME: Duration = Duration::from_secs(60 * 15);

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub token: String,
    pub role: String,
    pub expires_at: String,
}

impl Token {
    pub fn new(
        id: String,
        role: String,
        private_key: &AsymmetricSecretKey<V4>,
    ) -> Result<Self, ApplicationError> {
        let mut claims = Claims::new()?;
        claims.issuer("associme")?;
        claims.add_additional("id", id.clone())?;
        claims.add_additional("role", role.clone())?;
        claims.set_expires_in(&TOKEN_LIFETIME)?;

        let token = public::sign(private_key, &claims, None, None)?;
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::from_std(TOKEN_LIFETIME)?)
            .ok_or(ApplicationError::InternalServerError)?
            .to_rfc3339();

        Ok(Self {
            id,
            token,
            role,
            expires_at,
        })
    }
}
