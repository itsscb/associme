use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Member {
    pub id: Uuid,
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub member_id: Option<i32>,
    pub birthday: DateTime<Utc>,
    pub postalcode: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub membership_state: Membership,
    pub resignation_date: Option<DateTime<Utc>>,
    pub resignation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub changed_at: DateTime<Utc>,
    pub changed_by: String,
}

impl Member {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Membership {
    Active,
    Passive,
    Pending,
    Resigned,
    None,
}

impl From<String> for Membership {
    fn from(membership: String) -> Self {
        match membership.as_str().to_lowercase().as_str() {
            "active" => Self::Active,
            "passive" => Self::Passive,
            "pending" => Self::Pending,
            "resigned" => Self::Resigned,
            _ => Self::None,
        }
    }
}

impl Default for Membership {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Membership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let membership = match self {
            Self::Active => "active",
            Self::Passive => "passive",
            Self::Pending => "pending",
            Self::Resigned => "Resigned",
            Self::None => "none",
        };
        write!(f, "{membership}")
    }
}
