pub mod service {
    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
        PgConnection,
    };


    use crate::logbook::model;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

    pub struct LogInfoTable {
        connection: PooledPg,
    }

    use crate::schema::loginfo::dsl::*;

    impl LogInfoTable {
        pub fn new(connection: PooledPg) -> LogInfoTable {
            LogInfoTable { connection }
        }

        pub fn get_logbook_list(&mut self) -> Result<Vec<model::LogInfo>, diesel::result::Error> {
            // use schema::empires;

            let empire = loginfo
                .limit(5)
                .select(model::LogInfo::as_select())
                .load(&mut self.connection)
                .expect("Error loading posts");

            Ok(empire)
        }
    }
}
