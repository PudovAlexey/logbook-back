use chrono::NaiveDateTime;
use diesel::{deserialize::{Queryable, QueryableByName}, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::images::model::LogImageInfo;

#[derive(Serialize, Insertable, Deserialize, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogInfo {
   pub id: i32,
   pub title: String,
   pub description: Option<String>,
   pub depth: f32,
   pub start_pressure: i32,
   pub end_pressure: i32,
   pub vawe_power: Option<f32>,
   pub side_view: Option<f32>,
   pub water_temperature: Option<f32>,
   pub start_datetime: NaiveDateTime,
   pub end_datetime: NaiveDateTime,
   pub user_id: uuid::Uuid,
   pub image_id: Option<i32>
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::loginfo)]

pub struct RequiredSelectListItems {
 pub id: i32,
 pub title: String,
 pub description: Option<String>,
 pub start_datetime: NaiveDateTime,  
 pub image_id: Option<i32>
}

#[derive(ToSchema, Deserialize, Serialize)]
pub struct LogList {
  pub id: i32,
  pub title: String,
  pub description: Option<String>,
  pub start_datetime: NaiveDateTime,  
  pub image_id: Option<i32>,
  pub image_data: Option<LogImageInfo>  
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateLogInfo {
  pub  title: String,
  pub  description: Option<String>,
  pub  depth: f32,
  pub  start_pressure: i32,
  pub  end_pressure: i32,
  pub  vawe_power: Option<f32>,
  pub  side_view: Option<f32>,
  pub  water_temperature: Option<f32>,
  pub  start_datetime: NaiveDateTime,
  pub  end_datetime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct CreateLogInfo {
  pub  title: String,
  pub  description: Option<String>,
  pub  depth: f32,
  pub  start_pressure: i32,
  pub  end_pressure: i32,
  pub  vawe_power: Option<f32>,
  pub  side_view: Option<f32>,
  pub  water_temperature: Option<f32>,
  pub  start_datetime: NaiveDateTime,
  pub  end_datetime: NaiveDateTime,
  pub user_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::loginfo)]
pub struct Organization {
pub id: i32,
}
