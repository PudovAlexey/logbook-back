use crate::error::{AppError, AppResult};
pub mod confirm_password;
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
                return Err(AppError::ValidationError);
            }
        }
    }

    pub fn validate_email(email: &str) -> AppResult<String> {
        match validate_email::validate_email(email) {
            Ok(m) => {
                return Ok(m);
            }
            Err(error) => {
                return Err(AppError::ValidationError);
            }
        }
    }
    pub fn compare_password(email: &str, compare_email: &str) -> AppResult<String> {
        if email == compare_email {
            return Ok(email.to_string());
        } else {
            return Err(AppError::ValidationError);
        }
    }
}
