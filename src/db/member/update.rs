use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::ApplicationError,
    models::member::{Member, Membership},
};

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct UpdateMember {
    pub id: Uuid,
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
    pub changed_by: Uuid,
}

#[tracing::instrument(skip(pool, member))]
pub async fn update(
    pool: &sqlx::PgPool,
    member: UpdateMember,
) -> Result<crate::models::member::Member, ApplicationError> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let mut first = true;
    let mut query = sqlx::QueryBuilder::new("UPDATE members SET ");
    if let Some(email) = member.email {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("email = ").push_bind(email);
    }

    if let Some(phone) = member.phone {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("phone = ").push_bind(phone);
    }

    if let Some(first_name) = member.first_name {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("first_name = ").push_bind(first_name);
    }

    if let Some(last_name) = member.last_name {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("last_name = ").push_bind(last_name);
    }

    if let Some(member_id) = member.member_id {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("member_id = ").push_bind(member_id);
    }

    if let Some(birthday) = member.birthday {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("birthday = ").push_bind(birthday.to_rfc3339());
    }

    if let Some(postalcode) = member.postalcode {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("postalcode = ").push_bind(postalcode);
    }

    if let Some(city) = member.city {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("city = ").push_bind(city);
    }

    if let Some(street) = member.street {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("street = ").push_bind(street);
    }

    if let Some(house_number) = member.house_number {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query.push("house_number = ").push_bind(house_number);
    }

    if let Some(membership_state) = member.membership_state {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query
            .push("membership_state = ")
            .push_bind(membership_state.to_string());
    }

    if let Some(resignation_date) = member.resignation_date {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query
            .push("resignation_date = ")
            .push_bind(resignation_date.to_rfc3339());
    }

    if let Some(resignation_reason) = member.resignation_reason {
        if first {
            first = false;
        } else {
            query.push(", ");
        }

        query
            .push("resignation_reason = ")
            .push_bind(resignation_reason);
    }

    if first {
        tx.rollback().await?;
        return Err(ApplicationError::MissingData("member data".to_string()));
    }

    query.push(", ");

    query.push("changed_by = ").push_bind(member.changed_by);
    query.push(", ");

    query.push("changed_at = ").push_bind(Utc::now());
    query.push(" WHERE id = ").push_bind(member.id);
    query.push(" RETURNING *");
    let updated_member = query.build_query_as::<Member>().fetch_one(&mut *tx).await?;
    tx.commit().await?;
    Ok(updated_member)
}
