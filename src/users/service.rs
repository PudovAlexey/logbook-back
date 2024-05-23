pub mod service {
    
    use crate::users::model::ResetUserPassword;
    use crate::{users::model::USER};
    use argon2::{
        password_hash::{SaltString}, Argon2, PasswordHasher,
    };
    
    use diesel::{
        prelude::*, r2d2::{ConnectionManager, PooledConnection}, result::Error, PgConnection
    };
    
    use rand_core::OsRng;
    

    use crate::{
        users::model::{CreateUserHandler, CreateUserHandlerQUERY, UpdateUserDataQuery},
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

            query
                .select(USER::as_select())
                .first(&mut self.connection)
                
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
                        let _eror_response = serde_json::json!({
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
                                is_verified.eq(false),
                            ))
                            .returning(id)
                            .get_result(&mut self.connection);

                        create_user
                    }
                    Err(_err) => Err(diesel::result::Error::RollbackTransaction),
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
             let updating_id =  diesel::update(users)
                    .filter(id.eq(user_id))
                    .set(is_verified.eq(true))
                    .returning(id)
                    .get_result(&mut self.connection)?;

                Ok(updating_id)
            } else {
                Err(diesel::result::Error::RollbackTransaction)
            }
        }

        pub fn update_user_handler(&mut self, user_id: uuid::Uuid, params: UpdateUserDataQuery) -> Result<uuid::Uuid, Error> {
            let existing_user = self.get_user_by_id(user_id);

            let _param = params.role.as_ref().map(|p| p.to_string());

            if existing_user.is_ok() {
                let update = diesel::update(users)
                .filter(id.eq(user_id))
                .set((
                    params.email.as_ref().map(|e| email.eq(e.as_str())),
                    params.name.as_ref().map(|n| name.eq(n.as_str())),
                    params.surname.as_ref().map(|s| surname.eq(s.as_str())),
                    params.patronymic.as_ref().map(|p| patronymic.eq(p.as_str())),
                    params.role.as_ref().map(|r| role.eq(r.as_str())),
                    params.avatar_id.map(|aid| avatar_id.eq(aid)),
                ))
                .returning(id)
                .get_result(&mut self.connection);

                update
            } else {
               let error =  existing_user.unwrap_err();

               Err(error)
            }
        }

        pub fn reset_user_password(&mut self, params: ResetUserPassword) -> Result<uuid::Uuid, Error> {
            let existing_user = self.get_user_by_id(params.user_id);
            let salt = SaltString::generate(&mut OsRng);

            let hashed_password = Argon2::default()
            .hash_password(params.password.as_bytes(), &salt)
            .map_err(|e| {
                let _eror_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Error while hashing password: {}", e)
                });
            })
            .map(|hash| hash.to_string());

            if existing_user.is_ok() {
                let update = diesel::update(users)
                .filter(id.eq(params.user_id))
                .set(password.eq(hashed_password.unwrap()))
                .returning(id)
                .get_result(&mut self.connection)
                .expect("Failed to delete user");

            Ok(update)
            } else {
                let error =  existing_user.unwrap_err();

                Err(error)
            }

        }

        pub fn remove_user_by_id(&mut self, user_id: uuid::Uuid) -> Result<uuid::Uuid, Error> {
            let existing_user = self.get_user_by_id(user_id);

            if existing_user.is_ok() {
                let delete = diesel::delete(users)
                .filter(id.eq(user_id))
                .returning(id)
                .get_result::<uuid::Uuid>(&mut self.connection)
                .expect("Failed to delete user");
                // .get_result(&mut self.connection);

                Ok(delete)
            } else {
                let error =  existing_user.unwrap_err();

                Err(error)
            }

        }

        pub fn remove_un_verified_users(&mut self) -> Result<Vec<uuid::Uuid>, Error> {
           let res = diesel::delete(users)
            .filter(is_verified.eq(false))
            .returning(id)
            .load::<uuid::Uuid>(&mut self.connection);


            res
        }
    }
}
