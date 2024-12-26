use crate::{errors::ApplicationError, hash_password, models::Account};

#[tracing::instrument(skip(pool))]
pub async fn create_account(
    pool: sqlx::PgPool,
    email: &str,
    password: &str,
) -> Result<Account, ApplicationError> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let account = sqlx::query_as!(
        Account,
        "INSERT INTO accounts (email, password_hash)
        VALUES ($1, $2)
        RETURNING *",
        email,
        hash_password(&password)?,
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(account)
}
