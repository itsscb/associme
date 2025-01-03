use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Uuid,
    pub user_agent: String,
    pub client_ip: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub refresh_token: String,
    pub is_blocked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
