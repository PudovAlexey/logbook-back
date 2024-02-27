use http::{StatusCode};
use axum::{Json};
use bcrypt::hash;
use std::time::{SystemTime, Duration};
use serde_json::{json, Value};
use jsonwebtoken::{self, encode, EncodingKey, Header};
use crate::common::load_env_variable;
use crate::users::model::{
        USER, UpsertUser,
        Claims,
        string_to_user_role,
    };

pub fn hash_password(body: &mut UpsertUser) -> Result<(), (StatusCode, Json<Value>)> {
if let Ok(hashed_password) = hash(&body.password, 12) {
    body.password = hashed_password;
    Ok(())
} else {
    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "err": "Failed to hash password"
    }))))
}
}

pub fn generate_token(user: &USER) -> Result<String, jsonwebtoken::errors::Error> {
    let role = string_to_user_role(user.clone().role);
    let expiration = SystemTime::now()
    .checked_add(Duration::from_secs(3600))
    .expect("Failed to calculate token expiration")
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("SystemTime before UNIX EPOCH")
    .as_secs() as i64;

    let claims = Claims {
        sub: user.email.clone(),
        role: role.clone(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(load_env_variable::load_env_variable("ENCRYPTION_KEY").as_ref()))
}