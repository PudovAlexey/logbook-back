use chrono::{NaiveDateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use crate::common::validators::{
    validate_email::validate_email,
    validate_password::validate_password,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


pub enum UserRole {
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
 pub avatar_id: Option<i32>,
}

pub struct ComparePassword<T> {
   pub user: T,
   pub confirm_password: String
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
     #[schema(example = "1995-05-15T14:30:00")]
    pub date_of_birth: NaiveDateTime,
    pub password: String,
    pub confirm_password: String
}

impl CreateUserHandlerQUERY {
    pub fn compare_password(self) -> bool {
        
        self.password == self.confirm_password
    }
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
    pub fn password_verify(self) -> Result<String, String> {
        validate_password(self.password)


    }

    pub fn email_verify(self) -> Result<String, String> {
        validate_email(self.email)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginUser {
   pub email: String,
   pub password: String
}

impl LoginUser {
    pub fn password_verify(self) -> Result<String, String> {
        validate_password(self.password)


    }

    pub fn email_verify(self) -> Result<String, String> {
        validate_email(self.email)
    }
}


#[derive(Serialize, Deserialize)]
pub struct UserRemoveSensitiveInfo {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub date_of_birth: NaiveDateTime,
    pub avatar_url: Option<String>,
}

impl From<USER> for UserRemoveSensitiveInfo {
    fn from(value: USER) -> Self {
        let USER { id, email, name, surname, patronymic, role, created_at, updated_at, date_of_birth, ..} = value;

       return UserRemoveSensitiveInfo {
            id,
            email,
            name,
            surname,
            patronymic,
            role,
            created_at,
            updated_at,
            date_of_birth,
            avatar_url: None
        }
    }
}

pub struct UpdateUserData {
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub role: Option<String>,
    pub updated_at: NaiveDateTime,
    pub avatar_id: Option<i32>,
}

pub struct UpdateUserDataQuery {
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub role: Option<String>,
    pub avatar_id: Option<i32>,
}

impl From<UpdateUserDataQuery> for UpdateUserData {
    fn from(value: UpdateUserDataQuery) -> Self {
        UpdateUserData {
            email: value.email,
            name: value.name,
            surname: value.surname,
            patronymic: value.patronymic,
            role: value.role,
            updated_at: Utc::now().naive_utc(),
            avatar_id: value.avatar_id,
        }
    }
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct ForgotPassword {
  pub email: String
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ResetPassword {
  pub secret_code: i32,  
  pub password: String,
  pub confirm_password: String,
}

impl ResetPassword {
    pub fn compare(self) -> bool {
        self.password == self.confirm_password
    }
}

#[derive(Serialize, Deserialize, ToSchema, Queryable)]
pub struct ResetUserPassword {
    pub user_id: uuid::Uuid,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VerifyUserCode {
    pub verify_code: i32,
}