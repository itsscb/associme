use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info, instrument};

use crate::db;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAuth {
    email: String,
    password: String,
}

// TODO: REMOVE THIS
#[instrument]
pub async fn show_login_form() -> Html<String> {
    Html(
        r#"
    <form action="/login" method="post">
        <input type="text" name="email" placeholder="Username">
        <input type="password" name="password" placeholder="Password">
        <input type="submit" value="Log in">
    </form>
    "#
        .to_string(),
    )
}

#[instrument(skip(pool))]
pub async fn create_account(
    State(pool): State<sqlx::PgPool>,
    Form(auth): Form<AccountAuth>,
) -> impl IntoResponse {
    (db::account::create_account(pool, &auth.email, &auth.password).await).map_or_else(
        |e| {
            error!("Failed to create account: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::response::Json(json!({ "error": "Internal server error" })),
            )
                .into_response()
        },
        |account| {
            info!("Created account with email: {}", auth.email);
            (StatusCode::CREATED, account.to_json()).into_response()
        },
    )
}
