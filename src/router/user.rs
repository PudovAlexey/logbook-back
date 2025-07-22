use axum::Router;

use crate::SharedStateType;

pub mod register_handler;
pub mod register_handler_dto;

pub fn user_routes(shared_state: SharedStateType) -> Router {
    Router::new()
}