use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyUserCodeDto {
    pub verify_code: i32,
}