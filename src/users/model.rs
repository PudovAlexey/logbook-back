use axum::Json;
use chrono::{NaiveDateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::{json, Value};


enum UserRole {
    USER,
    ADMIN,
}

impl UserRole {
    fn new(role: UserRole) -> String {
        match role {
            UserRole::ADMIN => String::from("ADMIN"),
            UserRole::USER => String::from("USER")
        }
    }
}

#[derive(Serialize, Insertable, Deserialize, Debug, Selectable, Queryable, ToSchema, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct USER {
 pub id: uuid::Uuid,
 pub email: String,
 pub name: String,
 pub surname: Option<String>,
 pub patronymic: Option<String>,
 pub role: String,
 pub created_at: NaiveDateTime,
 pub updated_at: NaiveDateTime,
 pub date_of_birth: NaiveDateTime,
 pub password: String,
 pub is_verified: bool,
}

#[derive(ToSchema, Debug)]
pub struct CreateUserHandler {
    pub email: String,
    pub name: String,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub date_of_birth: NaiveDateTime,
    pub password: String,
    pub is_verified: bool,
}

#[derive(ToSchema, Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserHandlerQUERY {
    pub email: String,
    pub name: String,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub date_of_birth: NaiveDateTime,
    pub password: String,
}

impl From<CreateUserHandlerQUERY> for CreateUserHandler {
    fn from(value: CreateUserHandlerQUERY) -> Self {
        CreateUserHandler {
            email: value.email,
            name: value.name,
            surname: value.surname,
            patronymic: value.patronymic,
            date_of_birth: value.date_of_birth,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            is_verified: false,
            password: value.password,
            role: UserRole::new(UserRole::USER),
        }
    }
}

impl CreateUserHandlerQUERY {
    pub fn password_verify(self) -> Result<String, (StatusCode, Json<Value>)> {
        let password = self.password;

        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_digit = false;
        let mut has_whitespace = false;
        let mut has_chars = 0;
        for c in password.chars() {
            has_chars += 1;
            if c.is_uppercase() {
                has_uppercase = true;
            } else if c.is_lowercase() {
                has_lowercase = true;
            } else if c.is_digit(10) {
                has_digit = true;
            } else if c.is_whitespace() {
                has_whitespace = true;
            }
        }
       let matching = has_chars >= 8 && has_uppercase && has_lowercase && has_digit && !has_whitespace;

       if matching {
        Ok(password)
       } else {
           Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "password is incorrect"}))))
       }


    }

    pub fn email_verify(self) -> Result<String, (StatusCode, Json<Value>)> {
        let email = self.email;
        let email_regex = Regex::new(r"^([a-z0-9-_+]([a-z0-9-_+.]*[a-z0-9-_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        if email_regex.is_match(&email) {
            Ok(email)
        } else {
            Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "email is incorrect"}))))
            
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginUser {
   pub email: String,
   pub password: String
}