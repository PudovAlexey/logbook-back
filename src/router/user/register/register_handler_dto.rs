use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserHandlerBody {
    pub email: String,
    pub name: String,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
     #[schema(example = "1995-05-15T14:30:00")]
    pub date_of_birth: NaiveDateTime,
    pub password: String,
    pub confirm_password: String
}