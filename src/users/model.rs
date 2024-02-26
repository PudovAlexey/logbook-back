use std::fmt::{self};
use chrono::NaiveDateTime;

pub enum UserRole {
    ADMIN,
    USER,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::ADMIN => write!(f, "ADMIN"),
            UserRole::USER => write!(f, "USER"),
        }
    }
}

pub fn string_to_user_role(role: String) -> UserRole {
    match role.as_str() {
        "ADMIN" => UserRole::ADMIN,
        "USER" => UserRole::USER,
        _ => UserRole::USER,
    }
}

pub struct USER {
pub id: i32,
pub email: String,
pub name: String,
pub surname: Option<String>,
pub patronymic: Option<String>,
pub role: String,
pub created_at: NaiveDateTime,
pub updated_at: NaiveDateTime,
pub date_of_birth: NaiveDateTime,
pub password: String,
}

pub struct LoginUser {
    pub email: String,
    pub password: String,
}