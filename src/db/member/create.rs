use crate::{errors::ApplicationError, models::member::Member};

use super::NewMember;

#[tracing::instrument(skip(pool))]
pub async fn create(
    pool: &sqlx::PgPool,
    member: NewMember,
) -> Result<crate::models::member::Member, ApplicationError> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let new_member = match member.member_id {
        Some(member_id) => {
            sqlx::query_as!(
                Member,
                "INSERT INTO members (phone, first_name, last_name, email, birthday, postalcode, city, street, house_number, membership_state, member_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING *",
                member.phone,
                member.first_name,
                member.last_name,
                member.email,
                member.birthday,
                member.postalcode,
                member.city,
                member.street,
                member.house_number,
                member.membership_state.to_string(),
                member_id,
            )
            .fetch_one(&mut *tx)
            .await?
        }
        None => {
            sqlx::query_as!(
                Member,
                "INSERT INTO members (phone, first_name, last_name, email, birthday, postalcode, city, street, house_number, membership_state)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING *",
                member.phone,
                member.first_name,
                member.last_name,
                member.email,
                member.birthday,
                member.postalcode,
                member.city,
                member.street,
                member.house_number,
                member.membership_state.to_string(),
            )
            .fetch_one(&mut *tx)
            .await?
        }
    };

    tx.commit().await?;
    Ok(new_member)
}
