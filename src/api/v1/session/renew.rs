// use axum::{
//     extract::State,
//     http::{HeaderMap, StatusCode},
//     response::IntoResponse,
//     Json,
// };
// use tracing::instrument;

// use crate::{models::token::TokenS, Config};

// #[derive(serde::Deserialize)]
// pub struct RenewReq {
//     pub refresh_token: String,
// }

// // TODO: Implement session renewal
// #[instrument(skip(config, renew_request))]
// pub async fn renew(
//     headers: HeaderMap,
//     State(config): State<Config>,
//     renew_request: Json<RenewReq>,
// ) -> Result<impl IntoResponse, StatusCode> {
//     let ip = headers
//         .get("X-Forwarded-For")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or("not found");
//     let user_agent = headers
//         .get("user-agent")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or("not found");

//     TokenS::validate(&renew_request.refresh_token, &config.keypair.public)
//         .map_err(|_| StatusCode::UNAUTHORIZED)?;

//     // let id = Token::get_claim(&renew_request.refresh_token, "id")
//     //     .ok_or(StatusCode::UNAUTHORIZED)?
//     //     .to_string();

//     // let uid = uuid::Uuid::from_str(&id).map_err(|_| StatusCode::UNAUTHORIZED)?;

//     Ok(())
//     // (db::session::revoke(&config.pool, &token.token).await).map_or_else(
//     //     |e| {
//     //         error!(token = &token.token, error = ?e);
//     //         (
//     //             StatusCode::INTERNAL_SERVER_ERROR,
//     //             axum::response::Json(json!({ "error": "Internal server error" })),
//     //         )
//     //             .into_response()
//     //     },
//     //     |()| {
//     //         info!(token = &token.token, "Session blocked");
//     //         (StatusCode::OK).into_response()
//     //     },
//     // )
// }
