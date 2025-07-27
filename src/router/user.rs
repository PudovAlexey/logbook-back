pub mod refresh_token;
pub mod register;
pub mod verify_user;

use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::SharedStateType;

use refresh_token::refresh_token_handler::refresh_token_handler;
use register::register_handler::register_handler;

#[derive(OpenApi)]
#[openapi(paths(
    register::register_handler::register_handler,
    refresh_token::refresh_token_handler::refresh_token_handler,
))]
pub struct UserApiDoc;

pub fn user_routes_v2(shared_state: SharedStateType) -> Router {
    Router::new()
        .route("/api/register", post(register_handler))
        .route(
            "/refresh-tokens2",
            axum::routing::post(refresh_token_handler),
        )
        .with_state(shared_state)
}
