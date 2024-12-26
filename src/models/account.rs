use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Email, PasswordHash, Role};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: Uuid,
    pub role: Role,
    pub email: Email,
    #[serde(skip_serializing)]
    pub password_hash: PasswordHash,
    #[serde(skip_serializing)]
    pub secret_key: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub verification_sent: Option<DateTime<Utc>>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
}

impl Account {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
