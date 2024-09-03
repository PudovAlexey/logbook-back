use diesel::{deserialize::Queryable, prelude::{Insertable, QueryableByName}, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Selectable, Queryable, ToSchema)]
#[diesel(table_name = crate::schema::dive_site)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DiveSite {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_verified: bool,
    pub depth_from: f32,
    pub depth_to: f32,
    pub level: i32,
    pub image_id: i32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::dive_site)]

pub struct RequiredSelectListItems {
 pub id: i32,
 pub title: String,
 pub description: Option<String>,
 pub image_id: i32,
}