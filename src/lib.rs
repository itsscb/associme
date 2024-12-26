pub mod api;
mod db;
pub mod errors;
mod models;
use api::v1::account::{create_account, show_login_form};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use errors::ApplicationError;
use tower_http::services::ServeDir;

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
                .route("/login", post(create_account))
                .with_state(pool),
        )
        .nest_service("/", ServeDir::new("frontend/dist/associme"))
        .fallback(get(Redirect::temporary("/")))
}
