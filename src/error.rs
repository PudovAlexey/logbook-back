use argon2::Error as ArgonError;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::Error as DieselError;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error as IoError, ErrorKind as IoErrorKind},
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation error: {0}")]
    ValidationError(String), // Улучшенное сообщение об ошибке

    #[error("User Allready Exists")]
    UserAllreadyExists,

    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError), // SQLx errors

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Database error: {0}")]
    RedisError(#[from] RedisError), // SQLx errors

    #[error("hashed password error: {0}")]
    HashedPasswordError(ArgonError), // SQLx errors
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::UserAllreadyExists => StatusCode::FORBIDDEN,
            AppError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) | AppError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::HashedPasswordError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for IoError {
    fn from(err: AppError) -> Self {
        IoError::new(IoErrorKind::Other, format!("AppError: {}", err))
    }
}

impl Display for ErrorResponse {
    /// Formats the error response for human-readable output.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Логирование ошибки (используя `tracing`)
        tracing::error!("Error occurred: {}", self);

        // Определение статуса ответа ("fail" для клиентских ошибок, "error" для серверных)
        let status = if self.status_code().is_client_error() {
            "fail"
        } else {
            "error"
        };

        // Создание JSON-ответа с полями `status` и `message`
        let error_response = ErrorResponse {
            status: status.to_string(),
            message: self.to_string(),
        };

        // Получение HTTP-кода ошибки
        let status_code = self.status_code();

        // Формирование HTTP-ответа (код состояния + JSON-тело)
        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&error_response).unwrap()))
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SuccessResponse<T> {
    data: T,
}

impl<T> SuccessResponse<T> {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        let success_response = SuccessResponse { data: self.data };

        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&success_response).unwrap(),
            ))
            .unwrap()
    }
}

pub fn into_response<T>(data: T) -> SuccessResponse<T> {
    SuccessResponse { data }
}

pub type AppResult<T> = Result<T, AppError>;
