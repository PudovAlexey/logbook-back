pub mod register_handler;

use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::SharedStateType;
pub mod register_handler_dto;

use register_handler::register_handler;


#[derive(OpenApi)]
#[openapi(paths(register_handler::register_handler))]
pub struct UserApiDoc;


pub fn user_routes_v2(shared_state: SharedStateType) -> Router {
    Router::new()
    .route("/api/register", post(register_handler))
    .with_state(shared_state)
}