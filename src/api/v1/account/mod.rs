use axum::response::Html;
use serde::Deserialize;
use tracing::instrument;

mod registration;
pub use registration::registration;

mod login;
pub use login::login;

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
    <form action="/api/account/login" method="post">
        <input type="text" name="email" placeholder="Username">
        <input type="password" name="password" placeholder="Password">
        <input type="submit" value="Log in">
    </form>
    "#
        .to_string(),
    )
}

// TODO: REMOVE THIS
#[instrument]
pub async fn show_registration_form() -> Html<String> {
    Html(
        r#"
    <form action="/api/account/registration" method="post">
        <input type="text" name="email" placeholder="Username">
        <input type="password" name="password" placeholder="Password">
        <input type="submit" value="Log in">
    </form>
    "#
        .to_string(),
    )
}
