pub mod service {
    use chrono::offset;
    use serde::Deserialize;
    use diesel::{
        associations::HasTable, prelude::*, r2d2::{ConnectionManager, PooledConnection}, PgConnection
    };


    use crate::logbook::model;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

    pub struct LogInfoTable {
        connection: PooledPg,
    }

    use crate::schema::loginfo::dsl::*;
    #[derive(Deserialize, Debug)]
    pub struct GetLogbookListParams {
       pub limit: Option<i64>,
       pub offset: Option<i64>
    }

    impl LogInfoTable {
        pub fn new(connection: PooledPg) -> LogInfoTable {
            LogInfoTable { connection }
        }


        pub fn get_logbook_list(&mut self, params: GetLogbookListParams) -> Result<Vec<model::LogInfo>, diesel::result::Error> {
            let limit = params.limit.unwrap_or(-1);
            let offset = params.offset.unwrap_or(-1);

            // println!("limit is {:?} offset is {:?}", params.limit, params.offset);
            let mut query = loginfo.into_boxed();

            if limit >= 0 {
                query = query.limit(limit);
            }

            if limit >= 0 {
                query = query.offset(offset);
            }

        Ok(query
            .select(model::LogInfo::as_select())
            .load(&mut self.connection)
            .expect("error to loading Logbook"))
        }
    }
}
