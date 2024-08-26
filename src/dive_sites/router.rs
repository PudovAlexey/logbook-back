use axum::{response::IntoResponse, Router};

use crate::common::db::ConnectionPool;

use axum::Json;

use serde_json::{json, Value};

use http::StatusCode;

const ENDPOINT: &str = "/dive_sites/";

pub fn dive_sites_routes(shared_connection_pool: ConnectionPool) -> Router {
    Router::new()
    .route(&format!("{}list", ENDPOINT), axum::routing::get(get_dive_site_list))
    .with_state(shared_connection_pool)
}

#[utoipa::path(
    get,
    path = format!("{}list", ENDPOINT),
    params(
        ("page" = Option<i64>, Query, description = "page"),
        ("page_size" = Option<i64>, Query, description = "page_size"),
        ("start_date" = Option<NaiveDateTime>, Query, description = "start_date"),
        ("end_date" = Option<NaiveDateTime>, Query, description = "end_date"),
        ("search_query" = Option<String>, Query, description = "search_query")
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [model::LogInfo])
    )
)]
pub async fn get_dive_site_list() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    Ok((StatusCode::OK, Json(json!({"data": "hello"}))))
}