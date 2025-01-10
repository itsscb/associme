use std::{fs::File, io::Read};

use tracing::{info, warn};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    let private_key = read_private_key().unwrap_or_else(|_| {
        warn!("Could not read Paseto PrivateKey, generating a temporary private key");
        paseto_maker::Maker::new_keypair().0
    });

    info!("Running Database migrations");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = associme::router(pool, &private_key);

    Ok(router.into())
}

fn read_private_key() -> Result<[u8; 64], Box<dyn std::error::Error>> {
    let mut file = File::open("paseto_private")?;
    let mut private_key = Vec::new();
    file.read_to_end(&mut private_key)?;

    Ok(private_key.try_into().expect("Invalid private key"))
}
