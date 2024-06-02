use diesel::{
    r2d2::{ConnectionManager, PooledConnection}, PgConnection
};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;


use crate::users::user_runtime_scheduler::user_runtime_scheduler;
pub async fn runtime_scheduler(connection: PooledPg) {
    user_runtime_scheduler(connection).await;
}