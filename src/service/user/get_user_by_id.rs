use diesel::prelude::*;

use std::sync::Arc;

use crate::error::AppResult;
use crate::schema::users;
use crate::SharedStateType;
use crate::service::user::dto::model::USER;

pub enum GetUserBy {
    Id(uuid::Uuid),
    Email(String),
}

pub trait IntoGetUserBy {
    fn into_get_user_by(self) -> GetUserBy;
}

impl IntoGetUserBy for uuid::Uuid {
    fn into_get_user_by(self) -> GetUserBy {
        GetUserBy::Id(self)
    }
}

impl IntoGetUserBy for String {
    fn into_get_user_by(self) -> GetUserBy {
        GetUserBy::Email(self)
    }
}

pub fn get_user_by<T: IntoGetUserBy>(shared_state: SharedStateType, data: T) -> AppResult<USER> {
    let db_connection = Arc::clone(&shared_state.db_pool);

    let query = match data.into_get_user_by() {
        GetUserBy::Id(uuid) => users::table.filter(users::id.eq(uuid)).into_boxed(),
        GetUserBy::Email(email) => users::table.filter(users::email.eq(email)).into_boxed(),
    };

    let mut pool: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
        db_connection.pool.get().expect("error to loading Logbook");

    Ok(query
        .select(USER::as_select())
        .first(&mut pool)
        .expect("error to loading Logbook"))
}
