pub mod router {

    use crate::common::error::{ErrorMessage, RecordsMessage};
    use crate::common::redis::{Redis, SetExpireItem};

    use lettre::message::header::ContentType;
    use ::time::Duration;
    use argon2::PasswordVerifier;
    use axum::extract::{Path, State};
    use axum::middleware;
    use axum::{http::header, response::IntoResponse, response::Response, Json, Router};
    use axum_extra::extract::cookie::{Cookie, SameSite};

    use crate::users::auth::auth;
    use http::StatusCode;
    use serde_json::{json, Value};
    use tokio::time::error;

    use crate::common::env::ENV;
    use crate::common::mailer::Mailer;

    use rand_core::OsRng;

    use jsonwebtoken::{encode, EncodingKey, Header};

    use crate::users::model::TokenClaims;

    use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher};

    use crate::users::service::service::UserTable;

    use crate::{
        common::db::ConnectionPool, users::model::CreateUserHandlerQUERY, users::model::LoginUser,
    };

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        let auth_middleware = middleware::from_fn_with_state(shared_connection_pool.clone(), auth);
        Router::new()
            .route("/register/", axum::routing::post(create_user_handler))
            .route(
                "/register/verify/:id",
                axum::routing::get(verify_user_handler),
            )
            .route("/login", axum::routing::post(login_user_handler))
            .route(
                "/logout",
                axum::routing::get(logout_user_handler).route_layer(auth_middleware),
            )
            .with_state(shared_connection_pool)
    }

    #[utoipa::path(
        post,
        path = "/register/",
        request_body = CreateUserHandlerQUERY
    )]

    pub async fn create_user_handler(
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<CreateUserHandlerQUERY>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let claims_error = RecordsMessage::new();

        let verify_email = body.clone().email_verify();

        let varify_password = body.clone().password_verify();

        let claims_error = if verify_email.is_err() {
            claims_error.add_key(
                String::from("email"),
                ErrorMessage {
                    path: vec![String::from("password")],
                    message: String::from("password is invalid"),
                },
            )
        } else {
            claims_error
        };

        let claims_error = if varify_password.is_err() {
            claims_error.add_key(
                String::from("email"),
                ErrorMessage {
                    path: vec![String::from("password")],
                    message: String::from("password is invalid"),
                },
            )
        } else {
            claims_error
        };

        let claims_error = claims_error
            .send()
            .map_err(|e| {
                return e;
            })
            .unwrap();

        let conntection = shared_state.pool.get().expect("Failed connection to POOL");

        let email = body.email.clone();

        let salt = SaltString::generate(&mut OsRng);
        let hashed_email = Argon2::default()
            .hash_password(body.email.as_bytes(), &salt)
            .map_err(|e| {

               println!("faliled to generate hashing pass")
            })
            .map(|hash| hash.to_string())
            .unwrap()
            .replace("/", ".");

        let hashed_key = format!("verify.{}", hashed_email);

        match UserTable::new(conntection).register_user_handler(body) {
            Ok(id) => {
                let expires_token = Redis::new().set_expire_item(SetExpireItem {
                    key: hashed_key.clone(),
                    value: id.to_string(),
                    expires: 3600,
                });

                println!("test {:#?}", expires_token);

                println!("hashed key {}", hashed_key);

                if expires_token.status == "success" {
                    let mailer = Mailer::new(Mailer {
                       header: ContentType::TEXT_HTML,
                       to: email.to_string(),
                       subject: "New subject".to_string(),
                       body: format!("go to link for complete registration <a href=\"http://localhost:{}/register/verify/{}\">http://localhost:{}/register/verify/{}</a>", ENV::new().APP_HOST ,hashed_key, ENV::new().APP_HOST ,hashed_key)
                   });
                    mailer.send();
                    Ok((StatusCode::OK, Json(json!({"test": id}))))
                } else {
                    let error = claims_error
                        .add_key(
                            String::from("email"),
                            ErrorMessage {
                                path: vec![String::from("password")],
                                message: String::from("password is invalid"),
                            },
                        )
                        .send()
                        .map_err(|e| {
                            return e;
                        })
                        .unwrap();

                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": error})),
                    ))
                }
            }
            Err(_error) => {
                let eror_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Error while hashing password: {}", _error)
                });

                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": eror_response})),
                ))
            }
        }
    }

    #[utoipa::path(
        get,
        path = "/register/verify/{id}",
        params(
            ("id" = i32, Path, description="Element id")
        ),
    )]

    pub async fn verify_user_handler(
        State(shared_state): State<ConnectionPool>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        println!("test");
        let connection = shared_state.pool.get().expect("Failed connection to POOL");
        let claims_user_id = Redis::new().get_item(id.clone());

        let claims_error = RecordsMessage::new();

        match claims_user_id {
            Ok(user_id) => {
                let uuid = uuid::Uuid::parse_str(&user_id).unwrap();

                match UserTable::new(connection).user_verify(uuid) {
                    Ok(user) => {
                        Redis::new().remove_item(id);
                        Ok((StatusCode::OK, Json(json!({"data": user}))))
                    }
                    Err(_) => Ok((StatusCode::OK, Json(json!({"test": "test"})))),
                }
            }
            Err(_) => {
                let claims_error = claims_error.add_key(
                    String::from("key_error"),
                    ErrorMessage {
                        path: vec![],
                        message: String::from("failed to read verify key"),
                    },
                );

                // let result = claims_error.send()
                // .map_err(|e| {
                //     e
                // })
                // .unwrap()
                // .misssmatched_error();

                Err((StatusCode::OK, Json(json!({"detail": "failed to read verify key"}))))
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/login",
        request_body = LoginUser,
    )]
    pub async fn login_user_handler(
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<LoginUser>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        let user = UserTable::new(connection)
            .get_user_by_email(body.email)
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("failed to get user reason: {}", e)
                });

                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

        let is_valid = match PasswordHash::new(&user.password) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(body.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true),
            Err(_) => false,
        };

        if !is_valid {
            let err_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid email or password",
            });

            return Err((StatusCode::BAD_REQUEST, Json(err_response)));
        };

        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user.id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ENV::new().JWT_SECRET.as_ref()),
        )
        .unwrap();

        let cookie = Cookie::build(token.to_owned())
            .path("/")
            .max_age(Duration::hours(1))
            .same_site(SameSite::Lax)
            .http_only(true)
            .finish();

        let mut res = Response::new(
            json!({
                "status": "success",
            })
            .to_string(),
        );

        res.headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        Ok(res)
    }

    pub async fn logout_user_handler() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let cookie = Cookie::build("")
            .path("/")
            .max_age(Duration::hours(-1))
            .same_site(SameSite::Lax)
            .http_only(true)
            .finish();

        let mut response = Response::new(json!({"status": "success"}).to_string());

        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        Ok(response)
    }
}
