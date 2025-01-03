use crate::{errors::ApplicationError, models::Session};

#[tracing::instrument(skip(pool))]
pub async fn list(
    pool: &sqlx::PgPool,
    account_id: &uuid::Uuid,
) -> Result<Vec<Session>, ApplicationError> {
    let sessions = sqlx::query_as!(
        Session,
        "SELECT * 
        FROM sessions 
        WHERE account_id = $1",
        account_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}
