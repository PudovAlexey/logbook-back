use std::collections::HashMap;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use http::StatusCode;


#[derive(Debug, Serialize)]
pub struct ErrorMessage {
  pub  path: Vec<String>,
  pub  message: String
}

#[derive(Debug, Serialize)]
pub struct RecordsMessage {
  pub detail: HashMap<String, ErrorMessage>
}

impl RecordsMessage {
    pub fn new() -> Self {
        RecordsMessage {
            detail: HashMap::new()
        }   
    }

    pub fn add_key(mut self, key: String, value: ErrorMessage) -> Self {
        self.detail.insert(key, value);

        RecordsMessage {
            detail: self.detail
        }  
    }

    pub fn send(self) -> Result<RecordsMessage, (StatusCode, Json<Value>)> {

        if self.detail.len() > 0 {
            Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"detail": self.detail}))))
        } else {
           return Ok(RecordsMessage {
            detail: self.detail
        })  
        }

    }

    pub fn misssmatched_error(self) -> (StatusCode, Json<Value>) {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"detail": "missmatched error"})))
    }
}