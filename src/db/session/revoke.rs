use crate::errors::ApplicationError;

#[tracing::instrument(skip(pool))]
pub async fn revoke(pool: &sqlx::PgPool, refresh_token: &str) -> Result<(), ApplicationError> {
    sqlx::query!(
        "UPDATE sessions 
        SET is_blocked = $1
        WHERE refresh_token = $2",
        true,
        refresh_token,
    )
    .execute(pool)
    .await?;

    Ok(())
}
