use crate::{errors::ApplicationError, hash_password, models::Account, verify_password};

#[tracing::instrument(skip(pool))]
pub async fn set_password(
    pool: &sqlx::PgPool,
    account_id: &uuid::Uuid,
    password: &str,
) -> Result<(), ApplicationError> {
    sqlx::query_as!(
        Account,
        "UPDATE accounts 
        SET password_hash = $1
        WHERE id = $2
        RETURNING *",
        hash_password(&password)?,
        account_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool))]
pub async fn create_account(
    pool: &sqlx::PgPool,
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
    pool: &sqlx::PgPool,
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
    .fetch_one(pool)
    .await?;

    Ok(account)
}

struct AccountAuth {
    id: uuid::Uuid,
    password_hash: String,
    role: String,
}

#[tracing::instrument(skip(pool))]
pub async fn login(
    pool: &sqlx::PgPool,
    email: &str,
    password: &str,
) -> Result<(uuid::Uuid, String), ApplicationError> {
    let result: AccountAuth = sqlx::query_as!(
        AccountAuth,
        "SELECT id, password_hash, role 
        FROM accounts 
        WHERE email = $1
        LIMIT 1",
        email,
    )
    .fetch_one(pool)
    .await?;

    if result.password_hash.is_empty() || !verify_password(password, &result.password_hash)? {
        return Err(ApplicationError::Unauthorized);
    }

    Ok((result.id, result.role))
}
