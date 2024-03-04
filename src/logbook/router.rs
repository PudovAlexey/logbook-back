pub mod router {

    use axum::Json;
    use axum::{extract::State, response::IntoResponse, Router};
    use serde_json::json;
    use serde_json::Value;

    use axum::extract::{Path, Query};

    use crate::logbook::model::{
        UpdateLogInfo,
        LogInfo
    };
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
            .route("/log_info/:id", axum::routing::put(update_loginfo_handler))
            .route("/log_info/", axum::routing::post(create_loginfo_handler))
            .with_state(shared_connection_pool)
    }

    pub(super) type Store = Mutex<Vec<LogInfo>>;
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
    
    #[utoipa::path(
        put,
        path = "/log_info/{id}",
        request_body = UpdateLogInfo,
        params(
            ("id" = i32, Path, description="Element id")
        ),
        // responses(
        //     (status = 200, description = "Logbook updated successfully", [model::LogInfo])
        // )
    )]
    pub async fn update_loginfo_handler(
        State(shared_state): State<ConnectionPool>,
        Path(id): Path<i32>,
        Json(body): Json<UpdateLogInfo>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state.pool.get().expect("Failed connection to POOL");

        match log_info_table::new(connection).update_loginfo_by_id(id, body) {
         Ok(updated_id) => {
            Ok((
                StatusCode::OK,
                Json(json!(updated_id)),
            ))
         }
         Err(error) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to read empire"})),
            ))
         }   
        }

        // Ok((StatusCode::OK, Json(json!({
        //     "id": params.id,
        //     "body": body,
        // }))))
    }

    #[utoipa::path(
        post,
        path = "/log_info/",
        request_body = model::CreateLogInfo,

    )]
    pub async fn create_loginfo_handler(
        State(shared_state): State<ConnectionPool>,
        Json(body): Json<UpdateLogInfo>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        Ok((StatusCode::OK, Json(json!({
            "body": body,
        }))))
    }


    // async fn get_logbook_list(Extension(params): Extension<LogInfoParams>) -> String {
    //     format!("Page size: {}, Page: {}", params.page_size, params.page)
    // }
}
