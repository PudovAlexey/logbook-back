use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{common::jwt::JWT, users::model::USER};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
    pub struct RefreshTokenParamsDto {
       pub id: uuid::Uuid,
       pub refresh_token: String,
    }

    #[derive(Debug, Serialize, Deserialize, ToSchema)]
    pub struct RefreshTokenParamsResponseDto {
        pub user: USER,
        pub token: JWT
    }