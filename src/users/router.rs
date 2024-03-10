pub mod router {
    use axum::extract::{Path, State};
    use axum::{response::IntoResponse, Json, Router};
    use http::StatusCode;
    use serde_json::{json, Value};
    use tokio::join;
    use crate::common::mailer::{
        Mailer
    };

    use crate::users::service::service::UserTable;

    use crate::{common::db::ConnectionPool, users::model::CreateUserHandlerQUERY};

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
            .route("/register/", axum::routing::post(create_user_handler))
            .route("/register/verify/:id", axum::routing::post(verify_user_handler))
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
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match UserTable::new(connection).user_verify(id) {
            Ok(user_id) => {
                Ok((StatusCode::OK, Json(json!({"test": "hello"}))))
            },
            Err(err) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to read empire"})),
                ))
            }
        }
    }
}