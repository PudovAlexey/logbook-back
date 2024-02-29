use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// #[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable,ToSchema)]
#[derive(Serialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogInfo {
    id: i32,
    title: String,
    description: Option<String>,
    depth: f32,
    start_pressure: i32,
    end_pressure: i32,
    vawe_power: Option<f32>,
    side_view: Option<f32>,
    water_temperature: Option<f32>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
    user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateLogInfo {
    title: String,
    description: Option<String>,
    depth: f32,
    start_pressure: i32,
    end_pressure: i32,
    vawe_power: Option<f32>,
    side_view: Option<f32>,
    water_temperature: Option<f32>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::loginfo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateLogInfo {
    title: String,
    description: Option<String>,
    depth: f32,
    start_pressure: i32,
    end_pressure: i32,
    vawe_power: Option<f32>,
    side_view: Option<f32>,
    water_temperature: Option<f32>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
}
