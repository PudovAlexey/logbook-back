use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ResponseStatus {
    Error,
    Success
}

#[derive(Serialize, Deserialize)]
pub struct ChatSocketResponseSchema {
   pub status: ResponseStatus,
   pub data: String
}