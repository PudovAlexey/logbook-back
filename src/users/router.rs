pub mod router {
    extern crate image;
    use crate::common::redis::{Redis, SetExpireItem};

    use multipart::server::Multipart;

    use ::time::Duration;
    use argon2::PasswordVerifier;
    use axum::extract::{Path, State};
    use axum::{http::header, response::IntoResponse, response::Response, Json, Router};
    use axum::{middleware, Extension};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use lettre::message::header::ContentType;
    use uuid::uuid;

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

    use crate::images::model::{
        CreateAvatarQuery,
        CreateImageQuery
    };

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
            .route("/set_avatar", axum::routing::post(set_user_avatar))
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
                "data": UserRemoveSensitiveInfo::from(user),
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

    pub async fn health_checker_handler() -> impl IntoResponse {
        const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

        let json_response = serde_json::json!({
            "status": "success",
            "message": MESSAGE
        });

        Json(json_response)
    }

    // pub async fn set_user_avatar(
    //     Extension(request): Extension<Option<axum::extract::Multipart<axum::body::box_body::RequestBody>>>,
    //     State(shared_state): State<ConnectionPool>,
    //     Json(body): Json<LoginUser>,
    // ) -> impl IntoResponse {
    //     Json(json!({"user": "data"}))
    // }

    use std::env;
    use std::fs::DirBuilder;
    use std::fs::File;
    use std::io::prelude::*;
    use crate::users::model::UpdateUserDataQuery;

    pub async fn set_user_avatar(
        // Path(id): Path<uuid::Uuid>,
        State(shared_state): State<ConnectionPool>,
        mut multipart: axum::extract::Multipart
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        // println!("{}", id);
        let user_id = uuid!("a829fee7-66de-4372-a81d-597f5a7f58be"); 
        let dir_path = "assets/avatar";
        let current_dir = env::current_dir().unwrap();
        while let Some(mut field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();

            if name == "file" {
                let mut data = Vec::new();

                while let Some(chunk) = field.chunk().await.unwrap() {
                    data.extend_from_slice(chunk.as_ref());
                }

                let filename = field.file_name().unwrap();
                let path = format!("assets/avatar/{}", &filename);

                let dir = current_dir.join(dir_path);
                let new_dir = DirBuilder::new().recursive(true).create(dir);

                if (new_dir.is_ok()) {
                    let mut file = File::create(&path).unwrap();
                    file.write_all(&data).unwrap();
                    let connection = shared_state.pool.get().expect("Failed connection to POOL");

                    let avatar_query = ImagesTable::new(connection)
                    .set_avatar(CreateAvatarQuery {
                        user_id,
                        image_data: CreateImageQuery {
                            path,
                            filename: String::from(filename)
                        }
                    });

                    if avatar_query.is_ok() {
                        let connectio2 = shared_state.pool.get().expect("Failed connection to POOL");

                        let update_user = UserTable::new(connectio2).update_user_handler(user_id, UpdateUserDataQuery {
                            avatar_id: Some(avatar_query.unwrap()),
                            email: None,
                            name: None,
                            surname: None,
                            patronymic: None,
                            role: None,
                        });
                        
                        let user = update_user.unwrap();

                        return Ok((StatusCode::OK, Json(json!({"data": user}))))
                    } else {
                        return Err((StatusCode::BAD_REQUEST, Json(json!({"detail": "error"}))))
                    }
                }
            }
        }
        Ok((StatusCode::OK, Json(json!({"error": "err"}))))
    }

    // pub async fn set_user_avatar(mut multipart: axum::extract::Multipart) {
    //     while let Some(mut field) = multipart.next_field().await.unwrap() {
    //         let name = field.name().unwrap().to_string();
    //         let mut data = Vec::new();

    //         while let Some(chunk) = field.next().await {
    //             data.extend_from_slice(&chunk.unwrap());
    //         }

    //         let path = format!("assets/{}", name);

    //         let mut file = File::create(&path).unwrap();
    //         file.write_all(&data).unwrap();

    //     }
    // }
}
