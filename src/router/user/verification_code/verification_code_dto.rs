use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CheckVerificationCodeBody {
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CheckVerificationCodeBodyResponse {
    pub verification_code_expires_in: i32,
}
