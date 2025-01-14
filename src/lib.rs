pub mod api;
mod db;
pub mod errors;
mod models;
use std::sync::Arc;

use api::{middleware, v1::account::show_registration_form};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
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

pub fn router(pool: sqlx::PgPool, private_key: &[u8; 64]) -> axum::Router {
    let config = Config::new(private_key, pool);

    Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/registration", post(api::v1::account::registration))
                    .route("/login", post(api::v1::account::login))
                    .nest(
                        "/session",
                        Router::new()
                            // .route("/refresh", post(api::v1::token::refresh))
                            // .route("/refresh", post(api::v1::token::refresh))
                            .route("/revoke", post(api::v1::session::revoke))
                            .route("/list", post(api::v1::session::list)),
                    )
                    .nest(
                        "/token",
                        Router::new().route("/public_key", get(api::v1::session::public_key)),
                    )
                    .nest(
                        "/account",
                        Router::new()
                            .route("/", get(api::v1::account::get_account))
                            .layer(axum::middleware::from_fn_with_state(
                                config.clone(),
                                middleware::authentication,
                            )),
                    )
                    .nest(
                        "/member",
                        Router::new()
                            .route("/", get(api::v1::member::list_members))
                            .route("/", post(api::v1::member::create_member))
                            .route("/:id", get(api::v1::member::get_member))
                            .route("/", patch(api::v1::member::update_member))
                            .route("/:id", delete(api::v1::member::delete_member))
                            .layer(axum::middleware::from_fn_with_state(
                                config.clone(),
                                middleware::admin,
                            ))
                            .layer(axum::middleware::from_fn_with_state(
                                config.clone(),
                                middleware::authentication,
                            )),
                    ),
            ),
        )
        // .route("/login", get(show_login_form))
        .route("/registration", get(show_registration_form))
        .nest_service(
            "/",
            ServeDir::new("frontend/dist/associme")
                .fallback(ServeFile::new("frontend/dist/associme/index.html")),
        )
        .with_state(config)
}

#[derive(Clone)]
struct Config {
    token_maker: Arc<paseto_maker::Maker<paseto_maker::version::V4, paseto_maker::purpose::Public>>,
    pool: sqlx::PgPool,
}

impl Config {
    pub fn new(private_key: &[u8; 64], pool: sqlx::PgPool) -> Self {
        let token_maker = paseto_maker::Maker::new(private_key).unwrap();
        Self {
            token_maker: Arc::new(token_maker),
            pool,
        }
    }
}
