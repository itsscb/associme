use crate::{errors::ApplicationError, models::member::Member};

#[tracing::instrument(skip(pool))]
pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<Member>, ApplicationError> {
    let members = sqlx::query_as!(Member, "SELECT * FROM members")
        .fetch_all(pool)
        .await?;

    Ok(members)
}
