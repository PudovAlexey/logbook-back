pub mod router {
    extern crate image;
    use crate::common::env::ENV;
    use crate::common::error_boundary::error_boundary::{
        self, BoundaryHandlers, FieldError, InsertFieldError,
    };
    use crate::common::jwt::{is_valid_token, remove_jwt_cookie, JWTToken, JWT};
    use crate::common::multipart::ImageMultipart;
    use crate::common::redis::{Redis, SetExpireItem};

    extern crate rand;
    

    use argon2::PasswordVerifier;
    use axum::extract::{Path, Query, State};
    use axum::middleware;
    use axum::{response::IntoResponse, response::Response, Json, Router};
    use lettre::message::header::ContentType;
    use rand::Rng;
    use serde::Deserialize;

    use crate::users::auth::auth;
    use http::StatusCode;
    use serde_json::{json, Value};

    use crate::common::mailer::Mailer;

    use argon2::{Argon2, PasswordHash};

    use crate::users::service::service::UserTable;

    use crate::{
        common::db::ConnectionPool, users::model::CreateUserHandlerQUERY, users::model::LoginUser,
    };

    use crate::images::service::service::ImagesTable;

    use crate::images::model::{CreateAvatarQuery, CreateImageQuery};

    use crate::users::model::{
        ResetPassword, ResetUserPassword, UpdateUserDataQuery, UserRemoveSensitiveInfo,
        VerifyUserCode,
    };
    use std::env;
    use std::fs::DirBuilder;
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Deserialize)]
    struct RefreshTokenParams {
        id: uuid::Uuid,
        refresh_token: String,
    }

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        let auth_middleware = middleware::from_fn_with_state(shared_connection_pool.clone(), auth);
        Router::new()
            .route(
                "/refresh-tokens",
                axum::routing::post(health_checker_handler),
            )
            .route("/register/", axum::routing::post(create_user_handler))
            .route(
                "/register/verify/:user_id",
                axum::routing::post(verify_user_handler),
            )
            .route("/login", axum::routing::post(login_user_handler))
            .route(
                "/logout",
                axum::routing::post(logout_user_handler).route_layer(auth_middleware.clone()),
            )
            .route(
                "/verification_code/:email",
                axum::routing::post(request_verification_code),
            )
            // .route("/forgot_password/", axum::routing::post(forgot_password_handler).route_layer(auth_middleware.clone()))
            .route(
                "/reset_password/:email",
                axum::routing::post(reset_password_handler),
            )
            // .route("/set_avatar/:id", axum::routing::post(set_user_avatar).route_layer(auth_middleware.clone()))
            .route("/get_avatarv2", axum::routing::get(get_user_avatar_v2))
            .route("/set_avatar/:id", axum::routing::post(set_user_avatar))
            .route(
                "/get_avatar/:id",
                axum::routing::post(get_user_avatar).route_layer(auth_middleware.clone()),
            )
            .route(
                "/remove_account/:id",
                axum::routing::get(remove_accaunt_handler).route_layer(auth_middleware.clone()),
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
        let verify_email = body.clone().email_verify();

        let varify_password = body.clone().password_verify();

        if !body.clone().compare_password() {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"detail": "werify password is incompatible"})),
            ));
        }

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

        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(100000..999999);

        match UserTable::new(conntection).register_user_handler(body) {
            Ok(id) => {
                let expires_token = Redis::new().set_expire_item(SetExpireItem {
                    key: format!("verify={}", { &id }),
                    value: random_number,
                    expires: 3600,
                });

                if expires_token.status == "success" {
                    let mailer = Mailer::new(Mailer {
                        header: ContentType::TEXT_HTML,
                        to: email.to_string(),
                        subject: "New subject".to_string(),
                        body: format!("your code is <span>{}</span>", { random_number }),
                    });

                    let _ = mailer.send();

                    Ok((StatusCode::OK, Json(json!({"data": id}))))
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
                    "message": format!("Failed create user: {}", _error)
                });

                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": eror_response})),
                ))
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/register/verify/{user_id}",
        request_body = VerifyUserCode,
        params(
            ("user_id" = i32, Path, description="Element id")
        ),
    )]

    pub async fn verify_user_handler(
        State(shared_state): State<ConnectionPool>,
        Path(user_id): Path<String>,
        Json(body): Json<VerifyUserCode>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");
        let claims_user_id = Redis::new().get_item(format!("verify={}", user_id));

        if claims_user_id.is_ok() {
            let mail_code: i32 = claims_user_id.unwrap().parse().expect("not a number");
            if mail_code == body.verify_code {
                let uuid = uuid::Uuid::parse_str(&user_id).unwrap();

                match UserTable::new(connection).user_verify(uuid) {
                    Ok(user) => {
                        let _ = Redis::new().remove_item(user_id);
                        Ok((StatusCode::OK, Json(json!({"data": user}))))
                    }
                    Err(_) => Ok((StatusCode::OK, Json(json!({"test": "test"})))),
                }
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": "email code is incorrect. Please try again"})),
                ));
            }
        } else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "failed to verify user"})),
            ));
        }

        // match claims_user_id {
        //     Ok(user_id) => {
        //         let mail_code: i32 = user_id.parse().expect("not a number");
        //         println!("{:?}", mail_code);

        //        if mail_code == body.verify_code {
        //         let uuid = uuid::Uuid::parse_str(&user_id).unwrap();

        //         match UserTable::new(connection).user_verify(uuid) {
        //             Ok(user) => {
        //                 Redis::new().remove_item(id);
        //                 Ok((StatusCode::OK, Json(json!({"data": user}))))
        //             }
        //             Err(_) => Ok((StatusCode::OK, Json(json!({"test": "test"})))),
        //         }
        //        } else {
        //         return Err((
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             Json(json!({"detail": "email code is incorrect. Please try again"})),
        //         ))
        //        }
        //     }
        //     Err(_) => {
        //         return Err((
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             Json(json!({"detail": "failed to verify user"})),
        //         ))
        //     }
        // }
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
        let mut error_boundary = error_boundary::ObjectError::new();

        let validators = vec![body.clone().password_verify(), body.clone().email_verify()];
        let mut fire_error = false;

        for (index, field) in validators.iter().enumerate() {
            if field.is_err() {
                let mut key = String::new();
                fire_error = true;
                match index {
                    0 => key.push_str("password"),
                    1 => key.push_str("email"),
                    _ => {}
                }
                error_boundary = error_boundary.insert(InsertFieldError {
                    key,
                    value: FieldError {
                        message: String::from("validation error"),
                        description: field.clone().unwrap_err(),
                    },
                });
            }
        }

        if fire_error {
            return Err(error_boundary.send_error());
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
            let error_boundary = error_boundary.insert(InsertFieldError {
                key: String::from("password"),
                value: FieldError {
                    message: String::from("login_failed"),
                    description: String::from("incorrect user or password"),
                },
            });
            return Err(error_boundary.send_error());
        };

        let token = JWT::new(user.id);

        let mut avatar_url: Option<String> = None;

        if user.avatar_id.is_some() {
            let id = user.avatar_id.unwrap();
            let image_connection = shared_state.pool.get().expect("Failed connection to POOL");

            let a = ImagesTable::new(image_connection).get_avatar_data(id);

            avatar_url = match a {
                Ok(data) => Some(data.path),
                _ => None,
            };
        }
        // todo
        let mut user = UserRemoveSensitiveInfo::from(user);
        user.avatar_url = avatar_url;

        let res = Response::new(
            json!({
                "data": {
                "data": user,
                "token": token,
                }
            })
            .to_string(),
        );

        let res_data = token.set_cookie(res);

        Ok(res_data)
    }

    #[utoipa::path(post, path = "/logout")]

    pub async fn logout_user_handler() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let res = Response::new(json!({"status": "success"}).to_string());

        let res = remove_jwt_cookie(res);

        Ok(res)
    }

    use http::HeaderMap;

    #[utoipa::path(
        post,
        path = "/refresh-tokens?id={id}&refresh_token={refresh_token}",
        params(
            ("id" = uuid::Uuid, Path, description="Element id"),
            ("refresh_token" = String, Path, description="Element id")
        ),
    )]

    async fn health_checker_handler(
        State(shared_state): State<ConnectionPool>,
        Query(params): Query<RefreshTokenParams>,
        _headers: HeaderMap,
    ) -> impl IntoResponse {
        let mut res: Json<Value> = Json(json!({"data": "success"}));
        let mut simple_error = error_boundary::SimpleError::new();
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        let check_user = UserTable::new(connection).get_user_by_id(params.id);

        match check_user {
            Ok(user) => {
                let token = params.refresh_token;

                let is_valid = is_valid_token(&token);

                if is_valid {
                    let token = JWT::new(user.id);

                    res = Json(json!({
                        "data": user,
                        "token": token
                    }));
                } else {
                    simple_error = simple_error
                        .insert(String::from("your token was not valid. please login again"));
                }
            }
            Err(_error) => {
                simple_error = simple_error.insert(String::from("failed to find user"));
            }
        }

        simple_error.send(res)
    }

    pub async fn set_user_avatar(
        Path(id): Path<uuid::Uuid>,
        State(shared_state): State<ConnectionPool>,
        multipart: axum::extract::Multipart,
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

            let img = image::open(&path).unwrap();

            if image.crop.width > 0 && image.crop.height > 0 {
                // Если параметры обрезки корректные, обрезаем изображение
                let cropped_img = img.clone().crop(
                    image.crop.x,
                    image.crop.y,
                    image.crop.width,
                    image.crop.height,
                );
                cropped_img.save(&path).unwrap();
            } else {
                // Если параметры обрезки не переданы, сохраняем оригинальное изображение
                img.save(&path).unwrap();
            }

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

                    if update_user.is_ok() {
                        Ok((StatusCode::OK, Json(json!({"data": update_user.unwrap()}))))
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"detail": "error"})),
                        ))
                    }
                }
                Err(error) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": error.to_string()})),
                )),
            };
        } else {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "failed to create directory"})),
            ))
        }
    }

    #[utoipa::path(
        post,
        path = "/get_avatar/{id}",
        params(
            ("id" = uuid::Uuid, Path, description="Element id"),
        ),
    )]

    pub async fn get_user_avatar(
        Path(id): Path<uuid::Uuid>,
        State(shared_state): State<ConnectionPool>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match ImagesTable::new(connection).get_user_avatar_data(id) {
            Ok(data) => Ok((
                StatusCode::OK,
                Json(
                    json!({"data": format!("{}{}:{}/{}", ENV::new().app_protocol, ENV::new().app_host, ENV::new().app_port, data) }),
                ),
            )),
            Err(error) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"err": error.to_string()})),
            )),
        }
    }

    // #[utoipa::path](
    //  post,
    //  path = "/forgot_password",
    //  request_body = ForgotPassword
    // )

    #[utoipa::path(
        post,
        path = "/verification_code/{email}",
        request_body = ForgotPassword
    )]

    pub async fn request_verification_code(
        Path(email): Path<String>,
        State(shared_state): State<ConnectionPool>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");
        let mut error_boundary = error_boundary::SimpleError::new();

        let can_try_again = Redis::new().get_item(String::from("verification_handler_expire"));

        if can_try_again.is_ok() {
            error_boundary =
                error_boundary.insert(String::from("request verification not expires yet"));

            return Err(error_boundary.send_error());
        }

        match UserTable::new(connection).get_user_by_email(email.clone()) {
            Ok(_user) => {
                let mut rng = rand::thread_rng();
                let random_number: u32 = rng.gen_range(100000..999999);

                let expires_token = Redis::new().set_expire_item(SetExpireItem {
                    key: format!("change_password={}", { &email }),
                    value: random_number,
                    expires: 3600,
                });

                if expires_token.status == "success" {
                    let mailer = Mailer::new(Mailer {
                        header: ContentType::TEXT_HTML,
                        to: email,
                        subject: String::from("enter these code to reset password"),
                        body: format!("your code is <span>{}</span>", { random_number }),
                    });

                    let res = mailer.send();

                    if res.is_ok() {
                        let allow_try_again_time = 60;

                        Redis::new().set_expire_item(SetExpireItem {
                            key: String::from("verification_handler_expire"),
                            value: true,
                            expires: allow_try_again_time,
                        });

                        Ok((
                            StatusCode::OK,
                            Json(json!({"data": {
                                "timer": allow_try_again_time
                            }})),
                        ))
                    } else {
                        error_boundary =
                            error_boundary.insert(String::from("failed to send message"));

                        Err(error_boundary.send_error())
                    }
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"detail": "error to load redis port"})),
                    ))
                }
            }
            Err(_error) => {
                error_boundary = error_boundary.insert(String::from("failed to find user"));

                Err(error_boundary.send_error())
            }
        }
    }

    #[utoipa::path(
        post,
        path = "/reset_password/{email}",
        request_body = ResetPassword
    )]

    pub async fn reset_password_handler(
        Path(email): Path<String>,
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<ResetPassword>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        println!("{}", email);

        match UserTable::new(connection).get_user_by_email(email.clone()) {
            Ok(user) => {
                let mut error_boundary = error_boundary::ObjectError::new();

                if body.clone().compare() {
                    let secret_key =
                        Redis::new().get_item(format!("change_password={}", { email }));

                    if secret_key.is_ok() {
                        let transform_key: i32 = secret_key.unwrap().parse().expect("not a number");

                        if transform_key == body.secret_code {
                            let is_valid = match PasswordHash::new(&user.password) {
                                Ok(parsed_hash) => Argon2::default()
                                    .verify_password(body.password.as_bytes(), &parsed_hash)
                                    .map_or(false, |_| true),
                                Err(_) => false,
                            };

                            if is_valid == false {
                                let connection2 =
                                    shared_state.pool.get().expect("Failed connection to POOL");
                                let result = UserTable::new(connection2).reset_user_password(
                                    ResetUserPassword {
                                        user_id: user.id,
                                        password: body.password,
                                    },
                                );

                                if result.is_err() {
                                    error_boundary = error_boundary.insert(InsertFieldError {
                                        key: String::from("secret_code"),
                                        value: FieldError {
                                            message: String::from("validation error"),
                                            description: String::from(
                                                "password is not compared with",
                                            ),
                                        },
                                    });
                                }
                            } else {
                                let mut error_boundary = error_boundary::SimpleError::new();

                                error_boundary = error_boundary
                                    .insert(String::from("password can't be like a same password"));

                                return Err(error_boundary.send_error());
                            }
                        } else {
                            error_boundary = error_boundary.insert(InsertFieldError {
                                key: String::from("secret_code"),
                                value: FieldError {
                                    message: String::from("validation error"),
                                    description: String::from("secret code is incorrect"),
                                },
                            });
                        }
                    } else {
                        error_boundary = error_boundary.insert(InsertFieldError {
                            key: String::from("secret_code"),
                            value: FieldError {
                                message: String::from("validation error"),
                                description: String::from("secret key has is incorrect type"),
                            },
                        });
                    }
                } else {
                    error_boundary = error_boundary.insert(InsertFieldError {
                        key: String::from("confirm_password"),
                        value: FieldError {
                            message: String::from("validation error"),
                            description: String::from(
                                "password is not compared with compare password",
                            ),
                        },
                    });
                }

                let res = Json(json!({"data": "password was successfully changed"}));

                error_boundary.send(res)
            }
            Err(_error) => {
                let mut error_boundary = error_boundary::SimpleError::new();

                error_boundary = error_boundary.insert(String::from("failed to find user"));

                Err(error_boundary.send_error())
            }
        }
    }

    // Вводим логин, получаем юзера на изменение пароля и записываем
    // в редис user_uuid на изменение пароля, отправляем хэш на изменение пароля
    // pub async fn forgot_password_handler(
    //     State(shared_state): State<ConnectionPool>,
    //     Json(body): Json<ForgotPassword>,
    // ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    //     let connection = shared_state.pool.get().expect("Failed connection to POOL");

    //     let error_boundary = error_boundary::SimpleError::new();

    //     match UserTable::new(connection).get_user_by_email(body.email) {
    //         Ok(user) => {
    //             let salt = SaltString::generate(&mut OsRng);
    //             let hashed_user_id = Argon2::default()
    //             .hash_password(user.id.as_bytes(), &salt)
    //             .map_err(|e| println!("faliled to generate hashing pass"))
    //             .map(|hash| hash.to_string())
    //             .unwrap()
    //             .replace("/", ".");

    //             let hashed_key = format!("password_reset.{}", {hashed_user_id});

    //         let expires_token = Redis::new().set_expire_item(SetExpireItem {
    //             key: hashed_key.clone(),
    //             value: user.id.to_string(),
    //             expires: 3600,
    //         });

    //           if  expires_token.status == "success" {
    //                 let mailer = Mailer::new(Mailer {
    //                     header: ContentType::TEXT_HTML,
    //                     to: user.email.to_string(),
    //                     subject: String::from("Logbook reset password"),
    //                     body: format!("go to link for refresh your password <a href=\"{}{}:{}/forgot_password/{}\">{}{}:{}/forgot_password/{}</a>",
    //                    ENV::new().APP_PROTOCOL,
    //                    ENV::new().APP_HOST,
    //                    ENV::new().APP_PORT,
    //                    hashed_key,

    //                    ENV::new().APP_PROTOCOL,
    //                    ENV::new().APP_HOST,
    //                    ENV::new().APP_PORT,
    //                    hashed_key
    //                 )
    //                 });

    //                 mailer.send();

    //                 Ok((StatusCode::OK, Json(json!({"data": user.id}))))
    //             } else {
    //                 Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"detail": "failed to set token in redis client"}))))
    //             }
    //         },

    //         Err(err) => {
    //             let error_boundary = error_boundary.insert(String::from("failed to get user"));

    //             Err(error_boundary.send_error())
    //         }
    //     }
    // }

    // юзер перехоит по ссылке из почты, открывается страница с восстановлением пароля // password, confirm_password
    // меняем пароль в бд

    use reqwest::header::HeaderValue;

    #[utoipa::path(
        get,
        path = "/get_avatarv2",
        // params(
        //     ("id" = uuid::Uuid, Path, description="remove_id")
        // )
    )]

    pub async fn get_user_avatar_v2(// State(shared_state): State<ConnectionPool>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let mut headers = HeaderMap::new();
        let header_string = format!("Api-Key {}", ENV::new().ya_cloud_token);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&header_string).unwrap(),
        );

        // let resp = reqwest::get("https://storage.api.cloud.yandex.net/storage/v1/buckets/useravatars123")
        // .await;

        let resp = reqwest::Client::new()
            // .get("https://storage.api.cloud.yandex.net/storage/v1/buckets/useravatars123")
            .get("https://storage.yandexcloud.net/useravatars123/1.jpg")
            .headers(headers)
            .send()
            .await;

        // let resp = reqwest::get("https://httpbin.org/ip")
        // .await;

        match resp {
            Ok(rs) => {
                let txt = rs.text().await.unwrap();

                println!("body = {txt:?}");

                Ok((
                    StatusCode::OK,
                    Json(json!({"detail": "werify password is incompatible"})),
                ))
            }
            Err(_) => Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"detail": "werify password is incompatible"})),
            )),
        }
    }

    #[utoipa::path(
        get,
        path = "/remove_account/{id}",
        params(
            ("id" = uuid::Uuid, Path, description="remove_id")
        )
    )]

    pub async fn remove_accaunt_handler(
        State(shared_state): State<ConnectionPool>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");
        let errors = error_boundary::SimpleError::new();

        match UserTable::new(connection).remove_user_by_id(id) {
            Ok(uuid) => Ok((
                StatusCode::OK,
                Json(json!({"data": format!("accaunt with id {} has been removed", uuid)})),
            )),
            Err(_error) => {
                let errors = errors.insert(String::from("failed to removing accaunn"));

                Err(errors.send_error())
            }
        }
    }
}
