use std::sync::Arc;

use diesel::prelude::*;

use crate::error::{AppError, AppResult};
use crate::{SharedState, SharedStateType};

pub struct PatchUserField {}

use crate::schema::users;

pub struct UserVerified {
    pub user_id: uuid::Uuid,
    pub verification_code: i32,
}

pub async fn user_verified(
    shared_state: SharedStateType,
    params: UserVerified,
) -> AppResult<uuid::Uuid> {
    let UserVerified {
        user_id,
        verification_code,
    } = params;

    let mut redis = Arc::clone(&shared_state.redis);

    let redis_code = redis.get_item(format!("verify={}", user_id))?;

    let redis_code: i32 = redis_code.parse().unwrap();

    if redis_code != verification_code {
        return Err(AppError::ValidationError(String::from(
            "verification code is incorrect",
        )));
    }

    // remove verification code from redis

    let shared_state_pool = Arc::clone(&shared_state);

    let mut pool = shared_state_pool
        .db_pool
        .pool
        .get()
        .expect("failed to get connection pool");

    let uuid: uuid::Uuid = diesel::update(users::table.find(user_id))
        .set(users::is_verified.eq(true))
        .returning(users::id)
        .get_result(&mut pool)?;

    Ok(uuid)
}
