use crate::error::{AppError, AppResult};
pub mod validate_email;
pub mod validate_password;

pub enum Validator {}

impl Validator {
    pub fn validate_password(password: &str) -> AppResult<String> {
        match validate_password::validate_password(password) {
            Ok(m) => {
                return Ok(m);
            }
            Err(error) => {
                return Err(AppError::ValidationError(error));
            }
        }
    }

    pub fn validate_email(email: &str) -> AppResult<String> {
        match validate_email::validate_email(email) {
            Ok(m) => {
                return Ok(m);
            }
            Err(error) => {
                return Err(AppError::ValidationError(error));
            }
        }
    }
    pub fn compare_password(password: &str, compare_password: &str) -> AppResult<String> {
        if password == compare_password {
            return Ok(password.to_string());
        } else {
            return Err(AppError::ValidationError(String::from("Emails do not match")));
        }
    }
}
