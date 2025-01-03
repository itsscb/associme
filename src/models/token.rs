use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use pasetors::{
    Public,
    claims::Claims as pClaims,
    keys::{AsymmetricPublicKey, AsymmetricSecretKey},
    public,
    token::UntrustedToken,
    version4::V4,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::ApplicationError;

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    id: String,
    token: String,
    #[serde(flatten, skip_serializing_if = "Claims::is_empty", default)]
    claims: Claims,
    expires_at: DateTime<Utc>,
}

impl Token {
    // TODO: Implement from_str
    // pub fn from_str(
    //     token: AsRef<str>,
    //     public_key: &AsymmetricPublicKey<V4>,
    // ) -> Result<Self, ApplicationError> {
    //     let untrusted_token: UntrustedToken<Public, V4> =
    //         pasetors::token::UntrustedToken::try_from(token.as_ref()).unwrap();

    //     let trusted_token =
    //         pasetors::version4::PublicToken::verify(public_key, &untrusted_token, None, None)
    //             .unwrap();

    //     let expires_at = trusted_token.
    //     let claims = trusted_token.payload_claims();
    //     let claims = Claims {
    //         claims: trusted_token.claims().clone(),
    //     };
    //     let token = token.as_ref().to_string();
    //     Self {
    //         id,
    //         token,
    //         claims,
    //         expires_at,
    //     }
    // }
    pub fn validate(
        token: &str,
        public_key: &AsymmetricPublicKey<V4>,
    ) -> Result<(), ApplicationError> {
        let untrusted_token: UntrustedToken<Public, V4> =
            pasetors::token::UntrustedToken::try_from(token)?;

        let _trusted_token =
            pasetors::version4::PublicToken::verify(public_key, &untrusted_token, None, None)?;

        Ok(())
    }

    pub fn get_claim(&self, key: &str) -> Option<&Value> {
        self.claims.claims.get(key)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub const fn expires_at(&self) -> &DateTime<Utc> {
        &self.expires_at
    }

    pub fn new(
        id: String,
        claims: Claims,
        lifetime: Duration,
        private_key: &AsymmetricSecretKey<V4>,
    ) -> Result<Self, ApplicationError> {
        let mut token_claims =
            pClaims::new().map_err(|err| ApplicationError::InternalServerError(err.into()))?;
        token_claims
            .issuer("associme")
            .map_err(|err| ApplicationError::InternalServerError(err.into()))?;
        token_claims
            .add_additional("id", id.clone())
            .map_err(|err| ApplicationError::InternalServerError(err.into()))?;

        claims.apply(&mut token_claims)?;

        token_claims
            .set_expires_in(&lifetime)
            .map_err(|err| ApplicationError::InternalServerError(err.into()))?;

        let token = public::sign(private_key, &token_claims, None, None)
            .map_err(|err| ApplicationError::InternalServerError(err.into()))?;
        let expires_at = chrono::Utc::now()
            .checked_add_signed(
                chrono::Duration::from_std(lifetime)
                    .map_err(|err| ApplicationError::InternalServerError(err.into()))?,
            )
            .ok_or(ApplicationError::InternalServerError(Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to calculate expiration time",
                ),
            )))?;

        Ok(Self {
            id,
            token,
            claims,
            expires_at,
        })
    }

    pub fn new_pair(
        id: String,
        claims: Claims,
        token_lifetime: Duration,
        refresh_lifetime: Duration,
        private_key: &AsymmetricSecretKey<V4>,
    ) -> Result<(Self, Self), ApplicationError> {
        let token = Self::new(id.clone(), claims, token_lifetime, private_key)?;
        let claims = Claims::new();
        let refresh_token = Self::new(id, claims, refresh_lifetime, private_key)?;
        Ok((token, refresh_token))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    claims: HashMap<String, Value>,
}

impl Claims {
    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }
    pub fn new() -> Self {
        Self {
            claims: HashMap::new(),
        }
    }

    pub fn add<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), ApplicationError> {
        let value = serde_json::to_value(value)?;
        self.claims.insert(key.to_string(), value);
        Ok(())
    }

    pub fn apply(&self, claims: &mut pClaims) -> Result<(), ApplicationError> {
        for (key, value) in &self.claims {
            claims
                .add_additional(key, value.clone())
                .map_err(|err| ApplicationError::InternalServerError(err.into()))?;
        }
        Ok(())
    }
}
