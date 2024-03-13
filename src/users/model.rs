use chrono::{NaiveDateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::common::security::{
    make_hashed_password
};

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

#[derive(ToSchema, Debug, Serialize, Deserialize)]
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