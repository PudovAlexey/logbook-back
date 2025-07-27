use crate::common::jwt::{is_valid_token, JWT};
use crate::error::{into_response, AppError};
use crate::router::user::refresh_token::refresh_token_dto::RefreshTokenParamsResponseDto;
use crate::service::user::get_user_by_id;
use crate::{
    error::{AppResult, SuccessResponse},
    router::user::refresh_token::refresh_token_dto::RefreshTokenParamsDto,
    SharedStateType,
};
use axum::extract::{Query, State};
use http::HeaderMap;

#[utoipa::path(
    post,
    path = "/api/refresh-tokens?id={id}&refresh_token={refresh_token}",
    tag = "user",
    params(
        ("id" = uuid::Uuid, Path, description="Id"),
        ("refresh_token" = String, Path, description="Refresh token")
    ),
    responses(
        (status = 200, description = "User successfully registered", body = SuccessResponse<RefreshTokenParamsResponseDto>),
    )
)]
pub async fn refresh_token_handler(
    State(shared_state): State<SharedStateType>,
    Query(params): Query<RefreshTokenParamsDto>,
    _headers: HeaderMap,
) -> AppResult<SuccessResponse<RefreshTokenParamsResponseDto>> {
    // let user = get_user_by_id::get_user_by_id(shared_state, params.id)?;

    // let token = params.refresh_token;

    // let is_valid = is_valid_token(&token);

    // if is_valid {
    //     let token = JWT::new(user.id);
    //     Ok(into_response(RefreshTokenParamsResponseDto {
    //         user: user,
    //         token: token,
    //     }))
    // } else {
    //     Err(AppError::ValidationError(String::from(
    //         "Invalid refresh token",
    //     )))
    // }

    todo!()
}
