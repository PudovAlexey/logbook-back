pub mod login;
pub mod refresh_token;
pub mod register;
pub mod verify_user;
pub mod verification_code;
pub mod logout;

use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::SharedStateType;

use login::login_user_handler::login_user_handler;
use refresh_token::refresh_token_handler::refresh_token_handler;
use register::register_handler::register_handler;
use verify_user::verify_user_handler::verify_user_handler;
use verification_code::verification_code_handler::verification_code_handler;
use logout::logout_user_handler::logout_user_handler;

#[derive(OpenApi)]
#[openapi(paths(
    register::register_handler::register_handler,
    refresh_token::refresh_token_handler::refresh_token_handler,
    verify_user::verify_user_handler::verify_user_handler,
    login::login_user_handler::login_user_handler,
    verification_code::verification_code_handler::verification_code_handler,
))]
pub struct UserApiDoc;

pub fn user_routes_v2(shared_state: SharedStateType) -> Router {
    Router::new()
        .route("/api/register", post(register_handler))
        .route(
            "/refresh-tokens2",
            axum::routing::post(refresh_token_handler),
        )
        .route("/api/login", post(login_user_handler))
        .route("/api/user/verify/{user_id}", post(verify_user_handler))
        .route("/api/user/verification_code/{email}", post(verification_code_handler))
        .route("/api/user/logout/", post(logout_user_handler))
        .with_state(shared_state)
}
