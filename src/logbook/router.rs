pub mod router {

    use axum::Json;
    use serde_json::Value;
    use serde_json::{json};
    use axum::{
        extract::{Path, State},
        Router,
        response::IntoResponse
    };

    use utoipa::{ToSchema};
    use crate::logbook::model;
    use crate::logbook::service::service::LogInfoTable as log_info_table;

    use crate::common::db::ConnectionPool;
    use serde::{Deserialize, Serialize};
    use tokio::sync::Mutex;
    use http::{StatusCode};

     pub fn logbook_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
        .route("/todo/:id", axum::routing::post(get_logbook_list))
        .with_state(shared_connection_pool)
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub enum TodoError {
        /// Todo already exists conflict.
        #[schema(example = "Todo already exists")]
        Conflict(String),
        /// Todo not found by id.
        #[schema(example = "id = 1")]
        NotFound(String),
        /// Todo operation unauthorized
        #[schema(example = "missing api key")]
        Unauthorized(String),
    }

    pub(super) type Store = Mutex<Vec<model::LogInfo>>;
    #[utoipa::path(
        post,
        path = "/todo/{id}",
        params(
            ("id" = i32, Path, description = "Todo database id"),
        ),
        request_body = model::LogInfo,
        responses(
            (status = 200, description = "List all todos successfully", body = [model::LogInfo])
        )
    )]
    pub async fn get_logbook_list(
        // State(store): State<Arc<Store>>,
        Path(id): Path<i32>,
        State(shared_state): State<ConnectionPool>,
        Json(todo): Json<Value>,
        
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get()
        .expect("Failed connection to POOL");

       match log_info_table::new(connection).get_logbook_list() {
        Ok(log_info) => {
            Ok((StatusCode::OK, Json(json!({"data": &log_info}))))
        },
        Err(err) => {
                        eprintln!("Error reading empire: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to read empire"}))))
        }
       }
    }

  pub fn my_test_function() {

    }
}