use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::ApplicationError, models::member::Membership};

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateMember {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub member_id: Option<i32>,
    pub birthday: Option<DateTime<Utc>>,
    pub postalcode: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub membership_state: Option<Membership>,
    pub resignation_date: Option<DateTime<Utc>>,
    pub resignation_reason: Option<String>,
    pub changed_by: Uuid,
}

#[tracing::instrument(skip(pool))]
pub async fn update(
    pool: &sqlx::PgPool,
    member: UpdateMember,
) -> Result<crate::models::member::Member, ApplicationError> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let mut query = String::from("UPDATE members SET ");
    let mut values = Vec::with_capacity(15);

    if let Some(email) = member.email {
        query.push_str("email = $1, ");
        values.push(email);
    }

    if let Some(phone) = member.phone {
        query.push_str("phone = $2, ");
        values.push(phone);
    }

    if let Some(first_name) = member.first_name {
        query.push_str("first_name = $3, ");
        values.push(first_name);
    }

    if let Some(last_name) = member.last_name {
        query.push_str("last_name = $4, ");
        values.push(last_name);
    }

    if let Some(member_id) = member.member_id {
        query.push_str("member_id = $5, ");
        values.push(member_id.to_string());
    }

    if let Some(birthday) = member.birthday {
        query.push_str("birthday = $6, ");
        values.push(birthday.to_rfc3339());
    }

    if let Some(postalcode) = member.postalcode {
        query.push_str("postalcode = $7, ");
        values.push(postalcode);
    }

    if let Some(city) = member.city {
        query.push_str("city = $8, ");
        values.push(city);
    }

    if let Some(street) = member.street {
        query.push_str("street = $9, ");
        values.push(street);
    }

    if let Some(house_number) = member.house_number {
        query.push_str("house_number = $10, ");
        values.push(house_number);
    }

    if let Some(membership_state) = member.membership_state {
        query.push_str("membership_state = $11, ");
        values.push(membership_state.to_string());
    }

    if let Some(resignation_date) = member.resignation_date {
        query.push_str("resignation_date = $12, ");
        values.push(resignation_date.to_rfc3339());
    }

    if let Some(resignation_reason) = member.resignation_reason {
        query.push_str("resignation_reason = $13, ");
        values.push(resignation_reason);
    }

    query.push_str("changed_by = $14, ");
    values.push(member.changed_by.to_string());

    query.push_str("updated_at = $15 WHERE id = $16 RETURNING *");
    values.push(Utc::now().to_rfc3339());
    values.push(member.id.to_string());

    let updated_member = sqlx::query_as::<_, crate::models::member::Member>(&query)
        .bind(&values[0])
        .bind(&values[1])
        .bind(&values[2])
        .bind(&values[3])
        .bind(&values[4])
        .bind(&values[5])
        .bind(&values[6])
        .bind(&values[7])
        .bind(&values[8])
        .bind(&values[9])
        .bind(&values[10])
        .bind(&values[11])
        .bind(&values[12])
        .bind(&values[13])
        .bind(&values[14])
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(updated_member)
}
