use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use tracing::instrument;

use crate::{
    db::{self, member::UpdateMember},
    errors::ApplicationError,
    models::member::Membership,
    Config,
};
use serde::Deserialize;

// #[allow(clippy::struct_field_names)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct UpdateMemberRequest {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub member_id: Option<i32>,
    #[serde(default)]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub postalcode: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub street: Option<String>,
    #[serde(default)]
    pub house_number: Option<String>,
    #[serde(default)]
    pub membership_state: Option<Membership>,
    #[serde(default)]
    pub resignation_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub resignation_reason: Option<String>,
}

impl UpdateMemberRequest {
    fn into_update_member(self, id: uuid::Uuid, changed_by: uuid::Uuid) -> UpdateMember {
        UpdateMember {
            id,
            email: self.email,
            phone: self.phone,
            first_name: self.first_name,
            last_name: self.last_name,
            member_id: self.member_id,
            birthday: self.birthday,
            postalcode: self.postalcode,
            city: self.city,
            street: self.street,
            house_number: self.house_number,
            membership_state: self.membership_state,
            resignation_date: self.resignation_date,
            resignation_reason: self.resignation_reason,
            changed_by,
        }
    }
}

#[instrument(skip(config, member))]
#[axum::debug_handler]
pub async fn update_member(
    State(config): State<Config>,
    Extension(account_id): Extension<uuid::Uuid>,
    Path(member_id): Path<String>,
    Json(member): Json<UpdateMemberRequest>,
) -> impl IntoResponse {
    let Ok(uuid) = uuid::Uuid::parse_str(&member_id) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "invalid member id"})),
        )
            .into_response();
    };

    let update_member = member.into_update_member(uuid, account_id);

    match db::member::update(&config.pool, update_member).await {
        Ok(member) => (StatusCode::OK, Json(json!({"member": member}))).into_response(),
        Err(e) => {
            tracing::error!(error = ?e, "failed to update member");
            match e {
                ApplicationError::NotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "member not found"})),
                )
                    .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "failed to update member"})),
                )
                    .into_response(),
            }
        }
    }
}
