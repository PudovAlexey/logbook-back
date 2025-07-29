use argon2::password_hash::SaltString;
use diesel::prelude::*;
use rand_core::OsRng;

use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use argon2::PasswordHasher;

use crate::error::AppError;
use crate::{error::AppResult, service::user::get_user_by_id::get_user_by, SharedStateType};

use crate::schema::users;

pub async fn reset_password(
    shared_state: SharedStateType,
    user_id: uuid::Uuid,
    secret_code: i32,
    email: String,
    password: String,
) -> AppResult<uuid::Uuid> {
    let redis = Arc::clone(&shared_state.redis);

    let shared_state_for_user = Arc::clone(&shared_state);
    let user = get_user_by(shared_state_for_user, email.clone())?;

    let secret_key = redis.get_item(format!("change_password={}", { email }))?;

    let transform_key: i32 = secret_key.parse().expect("not a number");

    if transform_key == secret_code {
        let is_valid = match PasswordHash::new(&user.password) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true),
            Err(_) => false,
        };

        if is_valid == false {
            let salt = SaltString::generate(&mut OsRng);

            let mut db_pool = Arc::clone(&shared_state.db_pool).pool.get().expect("msg");

            let hashed_password = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| {
                    let _eror_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Error while hashing password: {}", e)
                    });
                })
                .map(|hash| hash.to_string());

            let update: uuid::Uuid = diesel::update(users::table)
                .filter(users::id.eq(user_id))
                .set(users::password.eq(hashed_password.unwrap()))
                .returning(users::id)
                .get_result(&mut db_pool)?;

            return Ok(update);
        }
    }

    Err(AppError::ValidationError(
        (String::from("Password mast be not same")),
    ))
}
