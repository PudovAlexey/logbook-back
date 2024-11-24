use std::sync::Arc;

use crate::users::auth::auth;
use crate::users::model::USER;

use crate::dive_chat::service;
use crate::SharedState;
use axum::Json;
use axum::{response::IntoResponse, Router};
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use axum::extract::{Extension, Path, State};
use axum::middleware;

use http::StatusCode;

const CHAT_ENDPOINTS: &str = "/chat/";

pub fn chat_sites_routes(shared_state: Arc<SharedState>) -> Router {
    // let connection_pool = shared_state.connection_pool.clone();

    Router::new()
        .route(
            &format!("{}chats", CHAT_ENDPOINTS),
            axum::routing::get(get_chat_list).route_layer(middleware::from_fn_with_state(
                shared_state.connection_pool.clone(),
                auth,
            )),
        )
        .route(
            &format!("{}messages/:id", CHAT_ENDPOINTS),
            axum::routing::get(get_messages_by_id),
        )
        .route(
            &format!("{}create_chat", CHAT_ENDPOINTS),
            axum::routing::post(create_chat).route_layer(middleware::from_fn_with_state(
                shared_state.connection_pool.clone(),
                auth,
            )),
        )
        .route(
            &format!("{}create_message/:id", CHAT_ENDPOINTS),
            axum::routing::post(create_message).route_layer(middleware::from_fn_with_state(
                shared_state.connection_pool.clone(),
                auth,
            )),
        )
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
    State(shared_state): State<Arc<SharedState>>,
    Extension(user): Extension<USER>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state
        .connection_pool
        .pool
        .get()
        .expect("Failed connection to POOL");

    match service::get_chat_list_by_user_id(
        connection,
        service::ChatListByUserIdParams { id: user.id },
    ) {
        Ok(chats) => Ok((StatusCode::OK, Json(json!({"data": chats})))),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        )),
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
    let connection = shared_state
        .connection_pool
        .pool
        .get()
        .expect("Failed connection to POOL");
    // let connection = shared_state.pool.get().expect("Failed connection to POOL");

    match service::get_message_list_by_id(connection, service::GetMessageListByIdParams { id }) {
        Ok(messages) => Ok((StatusCode::OK, Json(json!({"data": messages})))),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        )),
    }
}

#[utoipa::path(
    post,
    path = format!("{}create_chat", CHAT_ENDPOINTS),
    request_body = CreateChatParams,
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<Message>)
    )
)]

pub async fn create_chat(
    Extension(_user): Extension<USER>,
    State(shared_state): State<Arc<SharedState>>,
    Json(body): Json<service::CreateChatParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let connection = shared_state
        .connection_pool
        .pool
        .get()
        .expect("Failed connection to POOL");

    match service::create_chat_mutation(connection, body) {
        Ok(messages) => Ok((StatusCode::OK, Json(json!({"data": messages})))),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        )),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MessageText {
    pub text: String,
}

#[utoipa::path(
    post,
    path = "/chat/create_message/{id}",
    request_body = MessageText,
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<Message>)
    )
)]

pub async fn create_message(
    Extension(user): Extension<USER>,
    State(shared_state): State<Arc<SharedState>>,
    Path(id): Path<i32>,
    Json(body): Json<MessageText>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let connection = shared_state
        .connection_pool
        .pool
        .get()
        .expect("Failed connection to POOL");

    let chat_producer = shared_state.kafka_chat_handler.clone();

    match service::create_message_mutation(
        connection,
        service::CreateMessageParams {
            chat_id: id,
            text: body.text,
            user,
        },
        chat_producer,
    ) {
        Ok(message_id) => Ok((StatusCode::OK, Json(json!({"data": message_id})))),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        )),
    }
}
