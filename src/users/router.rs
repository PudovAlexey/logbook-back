pub mod router {
    use core::time;

    use axum::body;
    use axum::extract::{Path, State};
    use ::time::Duration;
    use axum::{response::IntoResponse, response::Response, Json, Router, http::header};
    use axum_extra::extract::cookie::{
        Cookie, SameSite,
    };
    use chrono::Utc;
    use diesel::dsl::IntervalDsl;
    use diesel::expression::is_aggregate::No;
    use http::StatusCode;
    use serde_json::{json, Value};
    use tokio::join;
    use crate::common::env::ENV;
    use crate::common::mailer::{
        Mailer
    };
    use jsonwebtoken::{
        encode, EncodingKey, Header
    };
    use crate::schema::loginfo::user_id;
    use crate::users::model::TokenClaims;

    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    use crate::users::service::service::UserTable;

    use crate::{common::db::ConnectionPool, users::model::CreateUserHandlerQUERY, users::model::LoginUser};

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
            .route("/register/", axum::routing::post(create_user_handler))
            .route("/register/verify/:id", axum::routing::post(verify_user_handler))
            .route("/login", axum::routing::post(login_user_handler))
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
        let conntection = shared_state.pool.get()
        .expect("Failed connection to POOL");

        let email = body.email.clone();

        match UserTable::new(conntection).register_user_handler(body) {
            Ok(id) => {
                let mailer = Mailer::new(Mailer {
                    to: email.to_string(),
                    subject: "New subject".to_string(),
                    body: format!("go to link for complete registration /register/verify/{}", id)
                });

                mailer.send();

                Ok((StatusCode::OK, Json(json!({"test": id}))))

            },
            Err(error) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to read empire"})),
                ))
            }
        }

    }

    #[utoipa::path(
        post,
        path = "/register/verify/{id}",
        params(
            ("id" = i32, Path, description="Element id")
        ),
    )]

    pub async fn verify_user_handler(
        State(shared_state): State<ConnectionPool>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match UserTable::new(connection).user_verify(id) {
            Ok(_) => {
                Ok((StatusCode::OK, Json(json!({"test": "test"}))))
            },
            Err(err) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to read empire"})),
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

        let user = UserTable::new(connection).get_user_by_email(body.email)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("failed to get user reason: {}", e)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
        // .ok_or_else(|| {
        //     let error_response = serde_json::json!({
        //         "status": "fail",
        //         "message": "Invalid email or password",
        //     });
        //     (StatusCode::BAD_REQUEST, Json(error_response))
        // })?;

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
            &EncodingKey::from_secret(ENV::new().JWT_SECRET.as_ref())
        )
        .unwrap();

        let cookie = Cookie::build(token.to_owned())
        .path("/")
        .max_age(Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

        let mut res = Response::new(json!({
            "status": "success",
        }).to_string());

        res
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        Ok(res)

    }
}