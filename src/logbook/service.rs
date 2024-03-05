pub mod service {
    use crate::common::formatters::date;
    
    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
        result::Error,
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
        pub search_query: Option<String>,
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
                    title
                        .ilike(format!("%{}%", search_query))
                        .or(description.ilike(format!("%{}%", search_query))),
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

        pub fn update_loginfo_by_id(&mut self, update_id: i32, query: model::UpdateLogInfo) -> Result<i32, Error> {
            let model::UpdateLogInfo {
                title: tit, 
                description: descr, 
                depth: dep,
                start_pressure: start_pres,
                end_pressure: end_pres,
                vawe_power: vawe,
                side_view: side,
                water_temperature: water_temp,
                start_datetime: start_date,
                end_datetime: end_date,
                 ..} = query;

            let existing_user = self.get_loginfo_by_id(GetLogbookByIdParams {
                id: update_id
            });

            match existing_user {
                Ok(_) => {
                    let update_loginfo = diesel::update(loginfo).set((
                        title.eq(tit),
                        description.eq(descr),
                        depth.eq(dep),
                        start_pressure.eq(start_pres),
                        end_pressure.eq(end_pres),
                        vawe_power.eq(vawe),
                        side_view.eq(side),
                        water_temperature.eq(water_temp),
                        start_datetime.eq(start_date),
                        end_datetime.eq(end_date),
                    ))
                    .execute(&mut self.connection);

                    Ok(update_id)
                }
                Err(_) => Err(Error::NotFound),
            }
        }

        pub fn create_loginfo(&mut self, query: model::CreateLogInfo) -> Result<i32, Error> {
            let model::CreateLogInfo {
                title: tit,
                description: descr,
                depth: dep,
                start_pressure: start_pres,
                end_pressure: end_pres,
                vawe_power: vawe_pow,
                side_view: side,
                water_temperature: water_temp,
                start_datetime: start_date,
                end_datetime: end_date,
                user_id: user,
                ..} = query;
            
           let new_loginfo = diesel::insert_into(loginfo).values((
                title.eq(tit),
                description.eq(descr),
                depth.eq(dep),
                start_pressure.eq(start_pres),
                end_pressure.eq(end_pres),
                vawe_power.eq(vawe_pow),
                side_view.eq(side),
                water_temperature.eq(water_temp),
                start_datetime.eq(start_date),
                end_datetime.eq(end_date),
                user_id.eq(user),
            ))
            .returning(id)
            .get_result(&mut self.connection);

        new_loginfo
        }
    }
}
