pub mod router {
    use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
    use serde_json::{json, Value};

    use crate::db::ConnectionPool;

    use crate::users::model::LoginUser;


    #[utoipa::path(
        post,
        path = "/login",
        request_body = LoginUser,
        responses(
            (status = 200, description = "auth", body = String)
        )
    )]
    pub async fn login_user_handler(
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<LoginUser>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state
            .pool
            .get()
            .expect("Failed to acquire connection from pool");

            Ok((StatusCode::OK, Json(json!({"user": "id"}))))
    }
}
