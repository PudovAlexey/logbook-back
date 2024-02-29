use std::fmt::{self};
use chrono::NaiveDateTime;
use regex::Regex;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[derive(Serialize, Insertable, Debug, Selectable, Queryable, ToSchema, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
pub struct UpsertUser {
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: UserRole,
}


impl UpsertUser {
    pub fn is_valid_email(&self) -> bool {
        let email_pattern = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();

        email_pattern.is_match(&self.email)
    }
}