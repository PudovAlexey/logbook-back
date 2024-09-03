pub mod service {
    use crate::{logbook::log_list_query::{log_list_query, LogListParams}, users::model::USER};

    use chrono::NaiveDateTime;

    use diesel::{
        prelude::*, r2d2::{ConnectionManager, PooledConnection}, result::Error, sql_query, PgConnection
    };
    use serde::{Deserialize, Serialize};

    use crate::logbook::model;
    use crate::schema::loginfo::dsl::*;

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

    pub struct LogInfoTable {
        connection: PooledPg,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct SearchLogsParams {
        pub page: Option<i64>,
        pub page_size: Option<i64>,
        pub start_date: Option<NaiveDateTime>,
        pub end_date: Option<NaiveDateTime>,
        pub search_query: Option<String>,
    }

    #[derive(Deserialize, Debug, Serialize)]

    pub struct GetLogbookListParams {
        pub search_params: SearchLogsParams,
        pub user: USER,
    }

    // #[derive(Insertable, Deserialize, )]
    // pub struct LogInfo {
    //     id: i32,
    // }

    #[derive(Deserialize, Debug, Queryable)]
    pub struct GetLogbookByIdParams {
        pub id: i32,
    }

    #[derive(Deserialize, Debug, Queryable)]
    pub struct CREATELogInfoParams {
       pub body: model::CreateLogInfo,
       pub user_info: USER
    }


    impl LogInfoTable {
        pub fn new(connection: PooledPg) -> LogInfoTable {
            LogInfoTable { connection }
        }

        // pub fn get_test_logbook_list(&mut self) {
        //     let query = "SELECT id FROM loginfo;";

        //    let res: Vec<model::Organization> = sql_query(query)
        //     .load(&mut self.connection).unwrap();
        // }

        pub fn get_logbook_list(
            &mut self,
            params: GetLogbookListParams,
        ) -> Result<Vec<model::RequiredSelectListItems>, diesel::result::Error> {
            // let SearchLogsParams {
            //     page,
            //     page_size,
            //     start_datetime,
            //     end_datetime,
            // } = params.search_params;

            let SearchLogsParams {
                page_size, 
                page,
                start_date,
                end_date,
                search_query,
                 ..} = params.search_params;

            let USER {
                id: speciphic_id, ..
            } = params.user;

            let limit = page_size.unwrap_or(100);
            let offset = limit * (page.unwrap_or(1) - 1);

            // let limit = limit.unwrap_or(-1);
            // let offset = offset.unwrap_or(-1);
            // let search_query = search_query.unwrap_or(String::from(""));
            let query = log_list_query(LogListParams {
                search_value: search_query,
                user_id: speciphic_id,
                start_date,
                end_date,
                offset,
                limit
            });

            let res: Vec<model::RequiredSelectListItems> = sql_query(query)
            .load(&mut self.connection)
            .expect("error to loading Logbook");

            Ok(res)

            // let mut query = loginfo.into_boxed();

            // query = query
            //     .filter(user_id.eq(speciphic_id));
            // //     .filter(start_datetime.ge(start_date.unwrap_or_default()))
            // //     .filter(end_datetime.le(end_date.unwrap_or_default()))
            // //     .filter(title.eq(search_query.unwrap_or_default()))


            // if start_date.is_some() && end_date.is_some() {
            //     query = query
            //     .filter(start_datetime.ge(start_date.unwrap()))
            //     .filter(start_datetime.ge(end_date.unwrap()))
            // }

            // if search_query.is_some() {
            //     query = query
            //     .filter(title.eq(search_query.unwrap_or_default()))
            // }

            // Ok(query
            //     .offset(offset)
            //     .limit(limit)
            //     .select(model::RequiredSelectListItems::as_select())
            //     .load(&mut self.connection)
            //     .expect("error to loading Logbook"))
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

        pub fn update_loginfo_by_id(
            &mut self,
            update_id: i32,
            query: model::UpdateLogInfo,
        ) -> Result<i32, Error> {
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
                ..
            } = query;

            let existing_user = self.get_loginfo_by_id(GetLogbookByIdParams { id: update_id });

            match existing_user {
                Ok(_) => {
                    let _update_loginfo = diesel::update(loginfo.filter(id.eq(update_id)))
                        .set((
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

        pub fn create_loginfo(&mut self, query: CREATELogInfoParams) -> Result<i32, Error> {
            let CREATELogInfoParams {body, user_info} = query;

            let model::CreateLogInfo {
                title: tit,
                description: descr,
                depth: dep,
                start_pressure: start_pres,
                end_pressure: end_pres,
                vawe_power: vawe_pow,
                side_view: side,
                site_id: site,
                water_temperature: water_temp,
                start_datetime: start_date,
                end_datetime: end_date,
                ..
            } = body;

            let new_loginfo = diesel::insert_into(loginfo)
                .values((
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
                    site_id.eq(site),
                    user_id.eq(user_info.id),
                ))
                .returning(id)
                .get_result(&mut self.connection);

            new_loginfo
        }
    }
}
