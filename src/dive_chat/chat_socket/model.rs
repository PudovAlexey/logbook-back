use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum ResponseStatus {
    Error,
    Success,
}

#[derive(Serialize, Deserialize)]
pub struct ChatSocketResponseSchema {
    pub status: ResponseStatus,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageParams {
    pub room_id: i32,
    pub user_uuid: uuid::Uuid,
}
