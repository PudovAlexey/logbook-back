use diesel::prelude::*;

use std::sync::Arc;

use crate::error::AppResult;
use crate::schema::users;
use crate::users::model::USER;
use crate::SharedStateType;

pub fn get_user_by_id(shared_state: SharedStateType, user_id: uuid::Uuid) -> AppResult<USER> {
    let db_connection = Arc::clone(&shared_state.db_pool);

    let query = users::table.filter(users::id.eq(user_id));

    let mut pool = db_connection.pool.get().expect("error to loading Logbook");

    Ok(query
        .select(USER::as_select())
        .first(&mut pool)
        .expect("error to loading Logbook"))
}
