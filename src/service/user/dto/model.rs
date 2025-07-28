use chrono::NaiveDateTime;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, ToSchema)]
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
    fn from(user: USER) -> Self {
        UserRemoveSensitiveInfo {
            id: user.id,
            email: user.email,
            name: user.name,
            surname: user.surname,
            patronymic: user.patronymic,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            date_of_birth: user.date_of_birth,
            avatar_url: None, // Здесь можно установить реальный URL аватара, если он есть
        }
    }
}
