pub mod router {
    use axum::{response::IntoResponse, Json, Router};
    use http::StatusCode;
    use serde_json::{json, Value};
    use tokio::join;

    use crate::{common::db::ConnectionPool, users::model::CreateUserHandlerQUERY};

    pub fn user_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
            .route("/register/", axum::routing::post(create_user_handler))
            .with_state(shared_connection_pool)
    }

    #[utoipa::path(
        post,
        path = "/register",
        request_body = CreateUserHandlerQUERY
    )]

     pub async fn create_user_handler() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        Ok((StatusCode::OK, Json(json!({"test": "hello"}))))
    }
}