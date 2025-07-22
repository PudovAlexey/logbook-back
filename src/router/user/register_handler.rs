use axum::{extract::State, Json};

use crate::{error::AppResult, router::user::register_handler_dto::CreateUserHandlerBody, utils::validator::Validator, SharedStateType};

pub async fn register_handler(
    State(shared_state): State<SharedStateType>,
    Json(body): Json<CreateUserHandlerBody>,
) -> AppResult<String> {
    let CreateUserHandlerBody {
        password, 
        email, confirm_password, ..} = body;

        Validator::validate_password(&password)?;

        Validator::validate_email(&email)?;

        Validator::compare_password(&email, &confirm_password)?;



        todo!()
}