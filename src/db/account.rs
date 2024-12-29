use crate::{errors::ApplicationError, hash_password, models::Account, verify_password};

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

#[tracing::instrument(skip(pool))]
pub async fn get_account_by_email(
    pool: sqlx::PgPool,
    email: &str,
) -> Result<Account, ApplicationError> {
    let account = sqlx::query_as!(
        Account,
        "SELECT * 
        FROM accounts 
        WHERE email = $1
        LIMIT 1",
        email,
    )
    .fetch_one(&pool)
    .await?;

    Ok(account)
}

#[tracing::instrument(skip(pool))]
pub async fn login(
    pool: sqlx::PgPool,
    email: &str,
    password: &str,
) -> Result<(), ApplicationError> {
    let password_hash: String = sqlx::query_scalar!(
        "SELECT password_hash 
        FROM accounts 
        WHERE email = $1
        LIMIT 1",
        email,
    )
    .fetch_one(&pool)
    .await?;

    if password_hash.is_empty() || !verify_password(password, &password_hash)? {
        return Err(ApplicationError::Unauthorized);
    }

    Ok(())
}
