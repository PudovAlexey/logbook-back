

use axum::{
    extract::State, http::{header, StatusCode}, middleware::Next, response::{Response}, Json
};
use crate::common::env::ENV;
use crate::users::service::service::UserTable;
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation,
};
use crate::users::model::TokenClaims;
use axum_extra::extract::cookie::CookieJar;

#[derive(Debug)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

use crate::{
    common::db::ConnectionPool
};

pub async fn auth(
    cookie_jar: CookieJar,
    State(shared_state): State<ConnectionPool>,
    mut req: http::Request<axum::body::Body>,
    next: Next,
) -> Response {

    let token = cookie_jar
    .get("token")
    .map(|cookie| cookie.value().to_string())
    .or_else(|| {
        req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
    });

    let token = token
    .expect("Error token");

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(ENV::new().JWT_ACCESS_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })
    .expect("Error")
    .claims;

    let user_uuid = uuid::Uuid::parse_str(&claims.sub)
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string()
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })
    .expect("eror");

    let connection = shared_state.pool.get().expect("Failed connection to POOL");

    let user = UserTable::new(connection).get_user_by_id(user_uuid)
    .map_err(|e| {
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error fetching user from database: {}", e),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
    });

    let user = user.map_err(|_e| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })
    .expect("error");

    req.extensions_mut().insert(user);
    next.run(req).await

}