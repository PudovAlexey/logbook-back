pub mod service {
    
    use crate::common::error as CommonError;
    use crate::users::model::{self, UpsertUser, USER};
    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection}, ExpressionMethods, PgConnection, SelectableHelper
    };
    use serde_json::error;
    use crate::schema::users::dsl::*;

    // use crate::schema;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;
    pub struct UsersTable {
        connection: PooledPg,
    }


    impl UsersTable {
        pub fn new(connection: PooledPg) -> UsersTable {
            UsersTable { connection }
        }

        pub fn get_by_email(
            &mut self,
            other_email: String
        ) -> Result<Option<model::USER>, diesel::result::Error> {

            let query = users
            .filter(email.eq(other_email))
                .select(model::USER::as_select())
                .first(&mut self.connection)
                .expect("error to loading Logbook");


            Ok(Some((query)))
        }

        pub fn create_user(&mut self, create_user: UpsertUser) -> Result<USER, CommonError::QueryCustomError> {
            diesel::insert_into(users)
            .values((
                email.eq(&create_user.email),
            ))
            .get_result::<USER>(&mut self.connection)
            .map_err(|err| {
                CommonError::QueryCustomError::from_diesel_error(err, "while creating user")
            })
            // .select(model::USER::as_select())
            // .expect("error to loading Logbook");
        }
        
    }
}