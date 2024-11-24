use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dive_sites::model::DiveSite, images::model::LogImageInfo};

#[derive(
    Serialize,
    Insertable,
    Deserialize,
    Debug,
    Selectable,
    Queryable,
    ToSchema,
    Identifiable,
    Associations,
    PartialEq,
)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(belongs_to(DiveSite, foreign_key = site_id))]
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
    pub image_id: Option<i32>,
    pub site_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LogInfoWithDive {
    pub log_info: LogInfo,
    pub dive_site: DiveSite,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::loginfo)]

pub struct RequiredSelectListItems {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_datetime: NaiveDateTime,
    pub image_id: Option<i32>,
}

#[derive(ToSchema, Deserialize, Serialize)]
pub struct LogList {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_datetime: NaiveDateTime,
    pub image_id: Option<i32>,
    pub image_data: Option<LogImageInfo>,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateLogInfo {
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
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct CreateLogInfo {
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
    pub site_id: i32,
}

#[derive(
    Serialize, Deserialize, Queryable, QueryableByName, Insertable, Debug, Clone, Selectable,
)]
#[diesel(table_name = crate::schema::loginfo)]
pub struct Organization {
    pub id: i32,
}
