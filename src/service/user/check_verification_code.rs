use std::sync::Arc;

use lettre::message::header::ContentType;
extern crate rand;
use rand::Rng;

use crate::{
    common::redis::SetExpireItem,
    error::{AppError, AppResult},
    service::user::get_user_by_id::get_user_by,
    SharedStateType,
};

const ALLOW_TRY_AGAIN_TIME: i32 = 60;

pub async fn check_verification_code(
    shared_state: SharedStateType,
    email: String,
) -> AppResult<i32> {
    shared_state
        .redis
        .get_item(String::from("verification_handler_expire"))?;

    let shared_state_clone = Arc::clone(&shared_state);

    get_user_by(shared_state_clone, email.clone())?;

    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100000..999999);

    let redis = Arc::clone(&shared_state.redis);

    let expires_token = redis.set_expire_item(SetExpireItem {
        key: format!("change_password={}", { &email }),
        value: random_number,
        expires: 3600,
    });

    if expires_token.status == "success" {
        shared_state.mailer.send(
            email,
            ContentType::TEXT_HTML,
            format!("your code is <span>{}</span>", { random_number }),
        );

        return Ok(ALLOW_TRY_AGAIN_TIME);
    }

    return Err(AppError::InternalServerError);
}
