use std::sync::Arc;

use crate::{error::{into_response, SuccessResponse}, service::user::register::register_handler as register_handler_service};
use axum::{extract::State, Json};
use opentelemetry::KeyValue;

use crate::{
    error::AppResult, router::user::register_handler_dto::CreateUserHandlerBody,
    utils::validator::Validator, SharedStateType,
};
use serde_json::{json, Value};

#[utoipa::path(
        get,
        path = "api/register",
        request_body = String
    )]
pub async fn register_handler(
    State(shared_state): State<SharedStateType>,
    Json(body): Json<CreateUserHandlerBody>,
) -> AppResult<SuccessResponse<uuid::Uuid>> {
    let CreateUserHandlerBody {
        password,
        email,
        confirm_password,
        name,
        surname,
        patronymic,
        date_of_birth,
    } = body;

    let clone_shared_state = Arc::clone(&shared_state);

    clone_shared_state.metrics.request_counter.add(
        1,
        &[
            KeyValue::new("endpoint", "/api/register"),
            KeyValue::new("method", "GET"),
        ],
    );

    Validator::validate_password(&password)?;

    Validator::validate_email(&email)?;

    Validator::compare_password(&email, &confirm_password)?;

    let clone_shared_state = Arc::clone(&shared_state);

    let connection_uuid = register_handler_service(
        CreateUserHandlerBody {
            password,
            email,
            confirm_password,
            date_of_birth,
            patronymic,
            name,
            surname,
        },
        clone_shared_state,
    )?;

    Ok(into_response(connection_uuid))
}
