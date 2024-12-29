use std::{fs::File, io::Read};

use pasetors::{
    keys::{AsymmetricKeyPair, AsymmetricPublicKey, AsymmetricSecretKey, Generate},
    version4::V4,
};
use tracing::{info, warn};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    let _keypair = read_keypair().unwrap_or_else(|_| {
        warn!("Could not read Paseto Keypair, generating a temporary pair");
        generate_keypair().expect("Could not generate Paseto Keypair")
    });

    info!("Running Database migrations");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = associme::router(pool);

    Ok(router.into())
}

fn generate_keypair() -> Result<AsymmetricKeyPair<V4>, Box<dyn std::error::Error>> {
    let keypair = pasetors::keys::AsymmetricKeyPair::<V4>::generate()?;

    Ok(keypair)
}

fn read_keypair() -> Result<AsymmetricKeyPair<V4>, Box<dyn std::error::Error>> {
    let mut file = File::open("paseto_private")?;
    let mut private_key = Vec::new();
    file.read_to_end(&mut private_key)?;

    let private_key = AsymmetricSecretKey::<V4>::from(&private_key)?;
    let mut file = File::open("paseto_public")?;
    let mut public_key = Vec::new();
    file.read_to_end(&mut public_key)?;

    let public_key = AsymmetricPublicKey::<V4>::from(&public_key)?;

    let keypair = AsymmetricKeyPair::<V4> {
        public: public_key,
        secret: private_key,
    };

    Ok(keypair)
}
