use axum::{response::IntoResponse, Router};

use crate::common::db::ConnectionPool;

use axum::{
    Json,
    extract::{
        State, Query,
    }
};

use serde_json::{json, Value};

use http::StatusCode;

use super::service;

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
        ("search_query" = Option<String>, Query, description = "search_query")
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [model::LogInfo])
    )
)]
pub async fn get_dive_site_list(
    State(shared_state): State<ConnectionPool>,
    Query(params): Query<service::SearchDiveSiteParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state.pool.get().expect("Failed connection to POOL");

   let res = service::get_dive_site_list(connection, params);

//    Ok(StatusCode::OK, Json(json!({"data": res.unwrap()})))
match res {
    Ok(data) => {
        Ok((StatusCode::OK, Json(json!({"data": data}))))
    },
    Err(error) => {
        Err((StatusCode::BAD_REQUEST, Json(json!({"err": error.to_string()}))))
    }
}
}