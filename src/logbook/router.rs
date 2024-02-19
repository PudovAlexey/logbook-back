pub mod router {
    use serde_json::Value;
    use serde_json::{json};
    use axum::{
        Router,
    };

    use crate::common::db::ConnectionPool;

     pub fn empires_route(shared_connection_pool: ConnectionPool) -> Router {
        Router::new()
        .route("/logbook", axum::routing::get(get_logbook_list))
        .with_state(shared_connection_pool)
    }

    pub async fn get_logbook_list() -> axum::Json<Value> {
        axum::Json(json!({
            "id": 1,
            "name": "Alex",
            "slogan": "Hello",
            "description": "Descr",
        }))
    }
}