pub mod router {

    use axum::Json;
    use serde_json::Value;
    use serde_json::{json};
    use axum::{
        extract::{State},
        Router,
        response::IntoResponse
    };

    use axum::extract::Query;

    use crate::logbook::model;
    use crate::logbook::service::service::{GetLogbookListParams, LogInfoTable as log_info_table};

    use crate::common::db::ConnectionPool;
    use tokio::sync::Mutex;
    use http::{StatusCode};

     pub fn logbook_routes(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
        .route("/log_info", axum::routing::get(get_logbook_list))
        .with_state(shared_connection_pool)
    }

    pub(super) type Store = Mutex<Vec<model::LogInfo>>;
    #[utoipa::path(
        get,
        path = "/log_info",
        params(
            ("offset" = Option<i64>, Query, description = "page"),
            ("limit" = Option<i64>, Query, description = "Page Size"),
        ),
        responses(
            (status = 200, description = "List all todos successfully", body = [model::LogInfo])
        )
    )]
    pub async fn get_logbook_list(
        State(shared_state): State<ConnectionPool>,
        Query(params): Query<GetLogbookListParams>,
        
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

        let connection = shared_state.pool.get()
        .expect("Failed connection to POOL");

       match log_info_table::new(connection).get_logbook_list(params) {
        Ok(log_info) => {
            Ok((StatusCode::OK, Json(json!({"data": &log_info}))))
        },
        Err(err) => {
                        eprintln!("Error reading empire: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to read empire"}))))
        }
       }
    }

    // async fn get_logbook_list(Extension(params): Extension<LogInfoParams>) -> String {
    //     format!("Page size: {}, Page: {}", params.page_size, params.page)
    // }
}