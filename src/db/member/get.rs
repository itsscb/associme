use crate::{errors::ApplicationError, models::member::Member};

#[tracing::instrument(skip(pool))]
pub async fn get_by_id(pool: &sqlx::PgPool, id: &str) -> Result<Member, ApplicationError> {
    let uuid = uuid::Uuid::parse_str(id).map_err(|_| ApplicationError::NotFound)?;
    let member = sqlx::query_as!(Member, "SELECT * FROM members WHERE id = $1", uuid,)
        .fetch_one(pool)
        .await?;

    Ok(member)
}
