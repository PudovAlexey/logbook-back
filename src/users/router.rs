pub mod router {
    extern crate image;
    use crate::common::jwt::{JWTToken, JWT};
    use crate::common::multipart::ImageMultipart;
    use crate::common::redis::{Redis, SetExpireItem};

    use ::time::Duration;
    use argon2::PasswordVerifier;
    use axum::extract::{Path, State};
    use axum::{http::header, response::IntoResponse, response::Response, Json, Router};
    use axum::{middleware};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use lettre::message::header::ContentType;

    use crate::users::auth::auth;
    use http::StatusCode;
    use serde_json::{json, Value};

    use crate::users::model::UserRemoveSensitiveInfo;

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

    use crate::images::service::service::ImagesTable;

    use crate::images::model::{CreateAvatarQuery, CreateImageQuery};

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        let auth_middleware = middleware::from_fn_with_state(shared_connection_pool.clone(), auth);
        Router::new()
            .route("/healthchecker", axum::routing::get(health_checker_handler))
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
            .route("/set_avatar/:id", axum::routing::post(set_user_avatar))
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
        let verify_email = body.clone().email_verify();

        let varify_password = body.clone().password_verify();

        if verify_email.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "failed to create user"})),
            ));
        }

        if varify_password.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "failed to create user"})),
            ));
        }

        let conntection = shared_state.pool.get().expect("Failed connection to POOL");

        let email = body.email.clone();

        let salt = SaltString::generate(&mut OsRng);
        let hashed_email = Argon2::default()
            .hash_password(body.email.as_bytes(), &salt)
            .map_err(|e| println!("faliled to generate hashing pass"))
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
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"detail": "failed to create user"})),
                    ));
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
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": "failed to verify user"})),
                ))
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

        let errors = vec![body.clone().password_verify(), body.clone().email_verify()];

        if (errors.iter().any(|x| x.is_err())) {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": {
                        "email": body.clone().password_verify(),
                        "password": body.clone().email_verify()
                    }
                })),
            ));
        }

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

        let token = JWT::new(user.id);

        let mut avatar_url: Option<String> = None;

        if user.avatar_id.is_some() {
            let id = user.avatar_id.unwrap();
            let image_connection = shared_state.pool.get().expect("Failed connection to POOL");

            let a = ImagesTable::new(image_connection)
            .get_avatar_data(id);

           avatar_url = match a {
                Ok(data) => Some(data.path),
                (_) => None
            };
        }

        let user = UserRemoveSensitiveInfo::from(user)
        .avatar_url = avatar_url;

        let mut res = Response::new(
            json!({
                "data": user,
                "token": token,
            })
            .to_string(),
        );

        let res_data = token.set_cookie(res);

        Ok(res_data)
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

    pub async fn health_checker_handler() -> impl IntoResponse {
        const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

        let json_response = serde_json::json!({
            "status": "success",
            "message": MESSAGE
        });

        Json(json_response)
    }

    use crate::users::model::UpdateUserDataQuery;
    use std::env;
    use std::fs::DirBuilder;
    use std::fs::File;
    use std::io::prelude::*;
    pub async fn set_user_avatar(
        Path(id): Path<uuid::Uuid>,
        State(shared_state): State<ConnectionPool>,
        mut multipart: axum::extract::Multipart,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let image = ImageMultipart::new(multipart).await;
        let dir_path = "assets/avatar";
        let current_dir = env::current_dir().unwrap();
        let path = format!("{}/{}", &dir_path, &image.filename);
        let dir = current_dir.join(dir_path);
        let new_dir = DirBuilder::new().recursive(true).create(dir);
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        if new_dir.is_ok() {
            let mut file = File::create(&path).unwrap();
            file.write_all(&image.image_vec).unwrap();
            let img = image::open(&path).unwrap().crop(
                image.crop.x,
                image.crop.y,
                image.crop.width,
                image.crop.height,
            );
            img.save(&path).unwrap();

            let avatar_query = ImagesTable::new(connection).set_avatar(CreateAvatarQuery {
                user_id: id,
                image_data: CreateImageQuery {
                    path,
                    filename: String::from(image.filename),
                },
            });

            return match avatar_query {
                Ok(avatar_id) => {
                    let connectio2 = shared_state.pool.get().expect("Failed connection to POOL");
                    let update_user = UserTable::new(connectio2).update_user_handler(
                        id,
                        UpdateUserDataQuery {
                            avatar_id: Some(avatar_id),
                            email: None,
                            name: None,
                            surname: None,
                            patronymic: None,
                            role: None,
                        },
                    );

                    
                    if (update_user.is_ok()) {
                        Ok((StatusCode::OK, Json(json!({"data": update_user.unwrap()}))))
                        
                    }  else {
                        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"detail": "error"}))))
                    }
                }
                Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"detail": error.to_string()})))),
            };
        } else {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "failed to create directory"})),
            ))
        }
    }
}
