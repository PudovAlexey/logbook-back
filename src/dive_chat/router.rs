use crate::users::auth::auth;
use crate::users::model::USER;

use crate::dive_chat::service;
use axum::Json;
use axum::{response::IntoResponse, Router};


use serde_json::json;
use serde_json::Value;

use axum::extract::{Extension, Path, State};
use axum::middleware;


use crate::common::db::ConnectionPool;
use http::StatusCode;

const CHAT_ENDPOINTS: &str = "/chat/";

pub fn chat_sites_routes(shared_connection_pool: ConnectionPool) -> Router {
    Router::new()
        .route(
            &format!("{}chats", CHAT_ENDPOINTS),
            axum::routing::get(get_chat_list).route_layer(middleware::from_fn_with_state(shared_connection_pool.clone(), auth))
        )
        .route(&format!("{}messages/:id", CHAT_ENDPOINTS), axum::routing::get(get_messages_by_id))
        .with_state(shared_connection_pool)
}

#[utoipa::path(
    get,
    path = format!("{}chats", CHAT_ENDPOINTS),
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<Chat>)
    )
)]

pub async fn get_chat_list(
    State(shared_state): State<ConnectionPool>,
    Extension(user): Extension<USER>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state.pool.get().expect("Failed connection to POOL");

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
    State(shared_state): State<ConnectionPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state.pool.get().expect("Failed connection to POOL");

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