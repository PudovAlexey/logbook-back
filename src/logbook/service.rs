pub mod service {
    use crate::common::formatters::date;
    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
        PgConnection,
    };
    use serde::Deserialize;

    use crate::logbook::model;
    use crate::schema::loginfo::dsl::*;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

    pub struct LogInfoTable {
        connection: PooledPg,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetLogbookListParams {
        pub limit: Option<i64>,
        pub offset: Option<i64>,
        pub search_query: Option<String>
    }

    #[derive(Deserialize, Debug)]
    pub struct GetLogbookByIdParams {
        pub id: i32,
    }

    impl LogInfoTable {
        pub fn new(connection: PooledPg) -> LogInfoTable {
            LogInfoTable { connection }
        }

        pub fn get_logbook_list(
            &mut self,
            params: GetLogbookListParams,
        ) -> Result<Vec<model::LogInfo>, diesel::result::Error> {
            let limit = params.limit.unwrap_or(-1);
            let offset = params.offset.unwrap_or(-1);
            let search_query = params.search_query.unwrap_or(String::from(""));

            let mut query = loginfo.into_boxed();

            let test = date::date::make_timestamp_from_string("31.12.2023");

            println!("timestamp {:?}", test);

            if limit >= 0 {
                query = query.limit(limit);
            }

            if limit >= 0 {
                query = query.offset(offset);
            }

            if search_query.len() > 0 {
                query = query.filter(
                    title.ilike(format!("%{}%", search_query))
                    .or(description.ilike(format!("%{}%", search_query)))
            )
            }

            Ok(query
                .select(model::LogInfo::as_select())
                .load(&mut self.connection)
                .expect("error to loading Logbook"))
        }

        pub fn get_loginfo_by_id(
            &mut self,
            params: GetLogbookByIdParams,
        ) -> Result<model::LogInfo, diesel::result::Error> {
            let query = loginfo.filter(id.eq(params.id));

            Ok(query
                .select(model::LogInfo::as_select())
                .first(&mut self.connection)
                .expect("error to loading Logbook"))
        }
    }
}
