pub mod router {

    use axum::Json;
    use axum::{extract::State, response::IntoResponse, Router};
    use serde_json::json;
    use serde_json::Value;

    use axum::extract::{Path, Query};

    use crate::logbook::model;
    use crate::logbook::service::service::{
        GetLogbookByIdParams, GetLogbookListParams, LogInfoTable as log_info_table,
    };

    use crate::common::db::ConnectionPool;
    use http::StatusCode;
    use tokio::sync::Mutex;

    pub fn logbook_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
            .route("/log_info", axum::routing::get(get_logbook_list))
            .route("/log_info/:id", axum::routing::get(get_logbook_by_id))
            .with_state(shared_connection_pool)
    }

    pub(super) type Store = Mutex<Vec<model::LogInfo>>;
    #[utoipa::path(
        get,
        path = "/log_info",
        params(
            ("offset" = Option<i64>, Query, description = "page"),
            ("limit" = Option<i64>, Query, description = "Page Size"),
            ("search_query" = Option<String>, Query, description = "seach value")
        ),
        responses(
            (status = 200, description = "List all todos successfully", body = [model::LogInfo])
        )
    )]
    pub async fn get_logbook_list(
        State(shared_state): State<ConnectionPool>,
        Query(params): Query<GetLogbookListParams>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match log_info_table::new(connection).get_logbook_list(params) {
            Ok(log_info) => Ok((StatusCode::OK, Json(json!({"data": &log_info})))),
            Err(err) => {
                eprintln!("Error reading empire: {:?}", err);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to read empire"})),
                ))
            }
        }
    }

    #[utoipa::path(
        get,
        path = "/log_info/{id}",
        params(
            ("id" = i32, Path, description="Element id")
        ),
        responses(
            (status = 200, description = "todo by id successfully", body= [model:: LogInfo])
        )
    )]
    pub async fn get_logbook_by_id(
        State(shared_state): State<ConnectionPool>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match log_info_table::new(connection).get_loginfo_by_id(GetLogbookByIdParams { id: id }) {
            Ok(log_item) => Ok((StatusCode::OK, Json(json!(log_item)))),
            Err(err) => {
                eprintln!("Error reading empire: {:?}", err);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to read empire"})),
                ))
            },
        }
    }

    // async fn get_logbook_list(Extension(params): Extension<LogInfoParams>) -> String {
    //     format!("Page size: {}, Page: {}", params.page_size, params.page)
    // }
}
