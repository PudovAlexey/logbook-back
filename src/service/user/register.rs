use diesel::prelude::*;

use diesel::ExpressionMethods;
use rand::Rng;

use crate::{
    error::AppResult,
    router::user::register_handler_dto::CreateUserHandlerBody,
    users::model::{USER},
    SharedStateType,
};

use crate::schema::users::dsl::*;
pub fn register_handler(
    params: CreateUserHandlerBody,
    shared_state: SharedStateType,
) -> AppResult<String> {
    let conntection = shared_state
        .db_pool
        .pool
        .get()
        .expect("Failed connection to POOL");

    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(100000..999999);

    let mut connection = shared_state
        .db_pool
        .pool
        .get()
        .expect("Failed connection to POOL");

    let existing_user: Option<USER> = users
        .filter(email.eq(&params.email))
        .select(USER::as_select())
        .first(&mut connection)
        .optional()
        .expect("error to loading Logbook");


    todo!()
}
