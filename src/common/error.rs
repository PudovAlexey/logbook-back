use core::fmt;
use std::fmt::write;

use diesel::result;

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    NotFount,
    Internal,
    UniqueViolation,
}

pub struct QueryCustomError {
    pub error_type: ErrorType,
    pub message: String,
}

impl QueryCustomError {
    pub fn new(message: &str, error_type: ErrorType) -> QueryCustomError {
        QueryCustomError { message: message.to_string(), error_type }
    }

    pub fn from_diesel_error(err: result::Error, context: &str) -> QueryCustomError {
        QueryCustomError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                result::Error::DatabaseError(db_err,_ ) => {
                    match db_err {
                        diesel::result::DatabaseErrorKind::UniqueViolation => ErrorType::UniqueViolation,
                        _ => ErrorType::Internal,
                    }
                }
                result::Error::NotFound => ErrorType::NotFount,
                _ => {
                    ErrorType::Internal
                }
            }

        )
    }
}

impl fmt::Display for QueryCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}