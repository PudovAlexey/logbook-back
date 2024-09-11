use diesel::prelude::*;
use chrono::NaiveDateTime;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, PartialEq, ToSchema, Identifiable, Associations)]
#[diesel(table_name = crate::schema::message)]
#[diesel(belongs_to(Chat, foreign_key = chat_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
   pub id: i32,
   pub text: String,
   pub chat_id: i32,
   pub created_at: Option<NaiveDateTime>,
   pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::chat)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::chat_user)]
#[diesel(belongs_to(Chat, foreign_key = chat_id))]
#[diesel(belongs_to(USER, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatUser {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: uuid::Uuid
}