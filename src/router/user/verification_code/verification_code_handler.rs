use axum::extract::{Path, State};

use crate::error::{into_response, AppResult, SuccessResponse};
use crate::router::user::verification_code::verification_code_dto::CheckVerificationCodeBodyResponse;
use crate::service::user::check_verification_code::{self, check_verification_code};
use crate::{
    router::user::verification_code::verification_code_dto::CheckVerificationCodeBody,
    SharedStateType,
};

#[utoipa::path(
        post,
        path = "/verification_code/{email}",
        request_body = CheckVerificationCodeBody,
    )]
pub async fn verification_code_handler(
    Path(email): Path<String>,
    State(shared_state): State<SharedStateType>,
) -> AppResult<SuccessResponse<CheckVerificationCodeBodyResponse>> {
    let check_verification_code = check_verification_code(shared_state, email).await?;

    Ok(into_response(CheckVerificationCodeBodyResponse {
        verification_code_expires_in: check_verification_code,
    }))
}
