pub mod router {
    use axum::Router;
    use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
    use serde_json::{json, Value};
    use axum::extract::{Path, Query};
    use bcrypt::verify;
    use crate::common::security::{
        generate_token,
        hash_password,
    };

    use crate::db::ConnectionPool;

    use crate::users::model::LoginUser;
    use crate::users::service::service::UsersTable;

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
        .route("/login", axum::routing::post(login_user_handler))
        .with_state(shared_connection_pool)
    }


    #[utoipa::path(
        post,
        path = "/login",
        request_body = LoginUser,
        responses(
            (status = 200, description = "auth", body = String)
        )
    )]

    async fn login_user_handler(
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<LoginUser>
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get()
        .expect("Failed to acquire connection from pool");

    match UsersTable::new(connection).get_by_email(body.email.clone()) {
        Ok(Some(user)) if body.email == user.email => {
            return if verify(&body.password, &user.password).unwrap_or(false) {
                if let Some(token) = generate_token(&user).ok() {
                    Ok((StatusCode::OK, Json(token)))
                } else {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to generate token"}))))
                }
            } else {
                Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Wrong password"}))))
            }
        }
        Ok(Some(_)) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))),
        Err(err) => {
            eprintln!("Error reading user: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to read user"}))))
        }
    }
            
        }
    }
