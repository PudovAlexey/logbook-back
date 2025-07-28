use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    common::jwt::JWT,
    service::user::dto::model::{UserRemoveSensitiveInfo, USER},
};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginUserBodyDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginUserResponseDto {
    pub user: UserRemoveSensitiveInfo,
    pub token: JWT,
}
