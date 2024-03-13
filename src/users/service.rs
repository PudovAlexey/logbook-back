pub mod service {
    use crate::{logbook::model, users::model::USER};
    use argon2::{
        password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };
    use diesel::{
        prelude::*, r2d2::{ConnectionManager, PooledConnection}, result::Error, sql_types::Uuid, PgConnection
    };
    use rand_core::OsRng;

    use crate::{
        schema::loginfo,
        users::model::{CreateUserHandler, CreateUserHandlerQUERY},
    };

    use crate::schema::users::dsl::*;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;
    pub struct UserTable {
        connection: PooledPg,
    }

    impl UserTable {
        pub fn new(connection: PooledPg) -> UserTable {
            UserTable { connection }
        }

        pub fn get_user_by_id(&mut self, user_id: uuid::Uuid) -> Result<USER, diesel::result::Error> {
            let query = users.filter(id.eq(user_id));

            Ok(query
                .select(USER::as_select())
                .first(&mut self.connection)
                .expect("error to loading Logbook"))
        }

        pub fn get_user_by_email(&mut self, user_email: String) -> Result<USER, diesel::result::Error> {
            let query = users.filter(email.eq(user_email));

            Ok(query
                .select(USER::as_select())
                .first(&mut self.connection)
                .expect("error to loading Logbook"))
        }

        pub fn register_user_handler(
            &mut self,
            params: CreateUserHandlerQUERY,
        ) -> Result<uuid::Uuid, diesel::result::Error> {
            let user_data: CreateUserHandler = CreateUserHandler::from(params);

            let existing_user: Option<USER> = users
                .filter(email.eq(&user_data.email))
                .select(USER::as_select())
                .first(&mut self.connection)
                .optional()
                .expect("error to loading Logbook");

            if existing_user.is_some() {
                Err(diesel::result::Error::NotFound)
            } else {
                let salt = SaltString::generate(&mut OsRng);
                let hashed_password = Argon2::default()
                    .hash_password(user_data.password.as_bytes(), &salt)
                    .map_err(|e| {
                        let eror_response = serde_json::json!({
                            "status": "fail",
                            "message": format!("Error while hashing password: {}", e)
                        });
                    })
                    .map(|hash| hash.to_string());

                match hashed_password {
                    Ok(pass) => {
                        let create_user = diesel::insert_into(users)
                            .values((
                                email.eq(user_data.email),
                                name.eq(user_data.name),
                                surname.eq(user_data.surname),
                                patronymic.eq(user_data.patronymic),
                                role.eq(user_data.role),
                                created_at.eq(user_data.created_at),
                                updated_at.eq(user_data.updated_at),
                                date_of_birth.eq(user_data.date_of_birth),
                                password.eq(pass),
                                is_verified.eq(true),
                            ))
                            .returning(id)
                            .get_result(&mut self.connection);

                        create_user
                    }
                    Err(err) => Err(diesel::result::Error::RollbackTransaction),
                }
            }
        }

        pub fn user_verify(&mut self, user_id: uuid::Uuid) -> Result<uuid::Uuid, Error> {
            let existing_user: Option<USER> = users
                .filter(id.eq(user_id))
                .select(USER::as_select())
                .first(&mut self.connection)
                .optional()
                .expect("error to loading Logbook");

            if existing_user.is_some() {
                diesel::update(users)
                    .filter(id.eq(user_id))
                    .set((is_verified.eq(true)))
                    .execute(&mut self.connection);

                Ok(user_id as uuid::Uuid)
            } else {
                Err(diesel::result::Error::RollbackTransaction)
            }
        }
    }
}
