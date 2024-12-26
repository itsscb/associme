use tracing::info;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    info!("Running Database migrations");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = associme::router(pool);

    Ok(router.into())
}
