use std::sync::Arc;

use crate::users::auth::auth;
use crate::users::model::USER;

use crate::dive_chat::service;
use crate::SharedState;
use axum::Json;
use axum::{response::IntoResponse, Router};


use serde_json::json;
use serde_json::Value;

use axum::extract::{Extension, Path, State};
use axum::middleware;

use http::StatusCode;

const CHAT_ENDPOINTS: &str = "/chat/";

pub fn chat_sites_routes(shared_state: Arc<SharedState>) -> Router {
    let connection_pool = shared_state.connection_pool.clone();

    Router::new()
        .route(
            &format!("{}chats", CHAT_ENDPOINTS),
            axum::routing::get(get_chat_list).route_layer(middleware::from_fn_with_state(connection_pool, auth))
        )
        .route(&format!("{}messages/:id", CHAT_ENDPOINTS), axum::routing::get(get_messages_by_id))
        .with_state(shared_state)
}

#[utoipa::path(
    get,
    path = format!("{}chats", CHAT_ENDPOINTS),
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<Chat>)
    )
)]

pub async fn get_chat_list(
    State(shared_state):State<Arc<SharedState>>,
    Extension(user): Extension<USER>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state.connection_pool.pool.get().expect("Failed connection to POOL");

    match service::get_chat_list_by_user_id(connection, service::ChatListByUserIdParams {
        id: user.id,
    }) {
        Ok(chats) => {
            Ok((
                StatusCode::OK, 
                Json(json!({"data": chats})),
            ))
        },
        Err(error) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({"error": error.to_string()})),
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/chat/messages/{id}",
    params(
        ("id" = i32, Path, description="Element id")
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<Message>)
    )
)]

pub async fn get_messages_by_id(
    State(shared_state): State<Arc<SharedState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // app_state.waiting_queue.read().await
    let connection = shared_state.connection_pool.pool.get().expect("Failed connection to POOL");
    // let connection = shared_state.pool.get().expect("Failed connection to POOL");

    match service::get_message_list_by_id(connection, service::GetMessageListByIdParams {
        id
    }) {
        Ok(messages) => {
            Ok((
                StatusCode::OK, 
                Json(json!({"data": messages})),
            ))
        },
        Err(error) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({"error": error.to_string()})),
            ))
        }
    }
    
}