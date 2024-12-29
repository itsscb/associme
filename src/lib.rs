pub mod api;
mod db;
pub mod errors;
mod models;
use api::v1::account::{login, registration, show_login_form, show_registration_form};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    routing::{get, post},
};
use errors::ApplicationError;
use tower_http::services::{ServeDir, ServeFile};

fn hash_password(password: &str) -> Result<String, ApplicationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, ApplicationError> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn router(pool: sqlx::PgPool) -> axum::Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/account",
                    Router::new()
                        .route("/registration", post(registration))
                        .route("/login", post(login)),
                )
                .with_state(pool),
        )
        // .route("/login", get(show_login_form))
        .route("/registration", get(show_registration_form))
        .nest_service(
            "/",
            ServeDir::new("frontend/dist/associme")
                .fallback(ServeFile::new("frontend/dist/associme/index.html")),
        )
}
