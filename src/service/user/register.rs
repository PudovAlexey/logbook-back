use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use diesel::prelude::*;

use diesel::ExpressionMethods;
use rand::Rng;

use crate::error::AppError;
use crate::schema::users;
use crate::{
    error::AppResult, router::user::register_handler_dto::CreateUserHandlerBody,
    users::model::USER, SharedStateType,
};
use rand_core::OsRng;

pub fn register_handler(
    params: CreateUserHandlerBody,
    shared_state: SharedStateType,
) -> AppResult<uuid::Uuid> {
    let CreateUserHandlerBody {
        password,
        email,
        name,
        surname,
        patronymic,
        date_of_birth,
        ..
    } = params;


    let db_connection = Arc::clone(&shared_state.db_pool);

    match users::table
        .filter(users::email.eq(&email))
        .select(USER::as_select())
        .first(&mut db_connection.pool.get().expect("error to loading Logbook"))
        .optional()
        .expect("error to loading Logbook")
    {
        Some(_) => Err(AppError::UserAllreadyExists),
        None => {
            let salt = SaltString::generate(&mut OsRng);

            let db_connection = Arc::clone(&shared_state.db_pool);

            let mut pool = db_connection.pool.get().expect("error to loading Logbook");

            match Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| {
                    let _eror_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Error while hashing password: {}", e)
                    });
                })
                .map(|hash| hash.to_string())
            {
                Ok(hashed_password) => {
                    let create_user = diesel::insert_into(users::table)
                        .values((
                            users::email.eq(email),
                            users::name.eq(name),
                            users::surname.eq(surname),
                            users::patronymic.eq(patronymic),
                            users::role.eq("User"),
                            users::created_at.eq(Utc::now().naive_utc()),
                            users::updated_at.eq(Utc::now().naive_utc()),
                            users::date_of_birth.eq(date_of_birth),
                            users::password.eq(hashed_password),
                            users::is_verified.eq(false),
                        ))
                        .returning(users::id)
                        .get_result(&mut pool)
                        .unwrap();

                    Ok(create_user)
                }
                Err(e) => Err(AppError::InternalServerError),
            }
        }
    }
}
