use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    error::{into_response, AppResult, SuccessResponse},
    router::user::verify_user::verify_user_dto::VerifyUserCodeDto,
    service::user::user_verified::{user_verified, UserVerified},
    SharedStateType,
};

    #[utoipa::path(
        post,
        tag="user",
        path = "api/user/verify/{user_id}",
        request_body = VerifyUserCodeDto,
        params(
            ("user_id" = uuid::Uuid, Path, description="Element id")
        ),
    )]

pub async fn verify_user_handler(
    State(shared_state): State<SharedStateType>,
    Path(user_id): Path<uuid::Uuid>,
    Json(body): Json<VerifyUserCodeDto>,
) -> AppResult<SuccessResponse<uuid::Uuid>> {
    let update_user = user_verified(
        shared_state,
        UserVerified {
            user_id: user_id,
            verification_code: body.verify_code,
        },
    )
    .await?;

    Ok(into_response(update_user))
}
