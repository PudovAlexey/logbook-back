use crate::{common::{
    env::ENV,
    error_boundary::
        error_boundary::{self, BoundaryHandlers}
    ,
}, SharedStateType};
use crate::users::model::TokenClaims;
use crate::users::service::service::UserTable;
use axum::{
    extract::State,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    extract::Request,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;

#[derive(Debug)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(shared_state): State<SharedStateType>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let error_boundary = error_boundary::SimpleError::new();

    let token = cookie_jar
        .get("access")
        .map(|cookie| {
           let mut str = cookie.value().to_string();
           str.pop();

           str
        })
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

    if token.is_some() {
        let claims = decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(ENV::new().jwt_access_secret.as_ref()),
            &Validation::default(),
        );

        match claims.is_ok() {
            true => {
                let claims = claims.unwrap().claims;

                let user_uuid = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
                    let json_error = ErrorResponse {
                        status: "fail",
                        message: "Invalid token".to_string(),
                    };

                    (StatusCode::UNAUTHORIZED, Json(json_error))
                });

                match user_uuid.is_ok() {
                    true => {
                        let connection =
                            shared_state.db_pool.clone().pool.get().expect("Failed connection to POOL");

                        let user = UserTable::new(connection)
                            .get_user_by_id(user_uuid.unwrap())
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
                                message: "The user belonging to this token no longer exists"
                                    .to_string(),
                            };
                            (StatusCode::UNAUTHORIZED, Json(json_error))
                        });

                        match user.is_ok() {
                            true => {
                            req.extensions_mut().insert(user.unwrap());


                            let response = next.run(req).await;
                            Ok(response)

                            }
                            false => {
                                let error_boundary = error_boundary
                                    .insert(String::from("failed to find user in database"));
                                Err(error_boundary.send_error())
                            }
                        }
                    }
                    false => {
                        let error_boundary =
                            error_boundary.insert(String::from("failed to unparse users uuid"));
                        Err(error_boundary.send_error())
                    }
                }
            }
            false => {
                let error_boundary = error_boundary.insert(String::from("token is incorrect"));
                Err(error_boundary.send_error())
            }
        }
    } else {
        let error_boundary = error_boundary.insert(String::from("failed to get token"));

        Err(error_boundary.send_error())
    }
}
