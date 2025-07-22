use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error as IoError, ErrorKind as IoErrorKind},
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("ValidationError")]
    ValidationError,
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
            AppError::ValidationError => StatusCode::BAD_REQUEST,
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

pub type AppResult<T> = Result<T, AppError>;
