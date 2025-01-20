use crate::{errors::ApplicationError, models::member::Member};

use super::NewMember;

#[tracing::instrument(skip(pool))]
pub async fn create(
    pool: &sqlx::PgPool,
    member: NewMember,
) -> Result<crate::models::member::Member, ApplicationError> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let created_by = if let Some(created_by) = member.created_by {
        created_by.to_string()
    } else {
        tx.rollback().await?;
        return Err(ApplicationError::MissingData("created_by".to_string()));
    };

    let new_member = match member.member_id {
        Some(member_id) => {
            sqlx::query_as!(
                Member,
                "INSERT INTO members (phone, first_name, last_name, email, birthday, postalcode, city, street, house_number, membership_state, member_id, created_by)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
                created_by,
            )
            .fetch_one(&mut *tx)
            .await.map_err(|err| {
                match &err {
                    sqlx::Error::Database(db_err) => {
                        if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) { // 23505 is the SQLSTATE code for unique violation
                            ApplicationError::Duplicate
                        } else {
                            ApplicationError::from(err)
                        }
                    }
                    _ => ApplicationError::from(err),
                }})?
        }
        None => {
            sqlx::query_as!(
                Member,
                "INSERT INTO members (phone, first_name, last_name, email, birthday, postalcode, city, street, house_number, membership_state, created_by)
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
                created_by
            )
            .fetch_one(&mut *tx)
            .await.map_err(|err| {
                match &err {
                    sqlx::Error::Database(db_err) => {
                        if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) { // 23505 is the SQLSTATE code for unique violation
                            ApplicationError::Duplicate
                        } else {
                            ApplicationError::from(err)
                        }
                    }
                    _ => ApplicationError::from(err),
                }})?
        }
    };

    tx.commit().await?;
    Ok(new_member)
}
