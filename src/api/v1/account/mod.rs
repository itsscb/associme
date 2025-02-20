use axum::response::Html;
use serde::Deserialize;
use tracing::instrument;

mod registration;
pub use registration::registration;

mod login;
pub use login::login;

mod get_account;
pub use get_account::get_account;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAuth {
    pub email: String,
    pub password: String,
}

// TODO: REMOVE THIS
#[instrument]
pub async fn show_registration_form() -> Html<String> {
    Html(
        r#"
    <form action="/api/registration" method="post">
        <input type="text" name="email" placeholder="Username">
        <input type="password" name="password" placeholder="Password">
        <input type="submit" value="Log in">
    </form>
    "#
        .to_string(),
    )
}
