extern crate image;
use chrono::{NaiveDateTime, Utc};
use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Insertable, Deserialize, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
  pub id: i32,
  pub path: String,
  pub filename: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Insertable, Deserialize, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::avatar)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Avatar {
   pub id: i32,
   pub image_id: i32,
   pub user_id: uuid::Uuid,
}

#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub struct CreateImageQuery {
  pub path: String,
  pub filename: String,
}

pub struct CreateImage {
  pub path: String,
  pub filename: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl From<CreateImageQuery> for CreateImage {
  fn from(value: CreateImageQuery) -> Self {
    CreateImage {
      path: value.path,
      filename: value.filename,
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
    }
  }
}


pub struct CreateAvatarQuery {
 pub image_data: CreateImageQuery,
 pub user_id: uuid::Uuid
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Eq, Debug, Clone)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Avatar, foreign_key = image_id))]
#[diesel(table_name = crate::schema::image)]
pub struct AvatarInfo {
    pub id: i32,
    pub path: String,
    pub filename: String,
}