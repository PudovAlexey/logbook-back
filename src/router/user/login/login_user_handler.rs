use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, Json};
use http::Response;
use serde_json::json;

use crate::{
    common::jwt::{JWTToken, JWT},
    error::{into_response, AppError, AppResult, SuccessResponse},
    router::user::login::login_user_dto::{LoginUserBodyDto, LoginUserResponseDto},
    service::user::{dto::model::UserRemoveSensitiveInfo, get_user_by_id::get_user_by},
    utils::validator::Validator,
    SharedStateType,
};

#[utoipa::path(
        post,
        tag = "user",
        path = "/api/login",
        request_body = LoginUserBodyDto,
    )]
pub async fn login_user_handler(
    State(shared_state): State<SharedStateType>,
    Json(body): Json<LoginUserBodyDto>,
) -> AppResult<SuccessResponse<LoginUserResponseDto>> {
    let LoginUserBodyDto {
        email, password, ..
    } = body;

    Validator::validate_password(&password)?;

    Validator::validate_email(&email)?;

    let user = get_user_by(shared_state, email)?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(AppError::ValidationError(String::from("Invalid password")));
    }

    let token = JWT::new(user.id);

    let user_without_sensitive_info = UserRemoveSensitiveInfo::from(user);

    let res = LoginUserResponseDto {
        user: user_without_sensitive_info,
        token: token.clone(),
    };

    let response = Response::new(json!(res).to_string());

    token.set_cookie(response);

    Ok(into_response(res))
}
