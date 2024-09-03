use serde::{Deserialize, Serialize};
use diesel::{
    prelude::*, r2d2::{ConnectionManager, PooledConnection}, sql_query, PgConnection
};


use super::{dive_site_list_query::{dive_site_list_query, DiveSiteParams}, model};

#[derive(Deserialize, Debug, Serialize)]
pub struct SearchDiveSiteParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search_query: Option<String>,
}

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_dive_site_list(
    connection: PooledPg,
    params: SearchDiveSiteParams,
) -> Result<Vec<model::RequiredSelectListItems>, diesel::result::Error> {

    let SearchDiveSiteParams {
        page_size, 
        page,
        search_query,
         ..} = params;

    let limit = page_size.unwrap_or(100);
    let offset = limit * (page.unwrap_or(1) - 1);

    let query = dive_site_list_query(DiveSiteParams {
        search_value: search_query,
        offset,
        limit
    });

    let mut connection = connection;

    let res: Vec<model::RequiredSelectListItems> = sql_query(query)
    .load(&mut connection)
    .expect("error to loading Logbook");

    Ok(res)
}