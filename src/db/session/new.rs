use crate::errors::ApplicationError;

#[allow(dead_code)]
struct NewSession {
    account_id: uuid::Uuid,
    refresh_token: String,
}

#[tracing::instrument(skip(pool))]
pub async fn new(
    pool: &sqlx::PgPool,
    account_id: &uuid::Uuid,
    user_agent: &str,
    client_ip: &str,
    refresh_token: &str,
    expires_at: &chrono::DateTime<chrono::Utc>,
) -> Result<(uuid::Uuid, String), ApplicationError> {
    let session: NewSession = sqlx::query_as!(
        NewSession,
        "INSERT INTO sessions (account_id, user_agent, client_ip, refresh_token, expires_at) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING account_id, refresh_token",
        account_id,
        user_agent,
        client_ip,
        refresh_token,
        expires_at,
    )
    .fetch_one(pool)
    .await?;

    Ok((session.account_id, session.refresh_token))
}
