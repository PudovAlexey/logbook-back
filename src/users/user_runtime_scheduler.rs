use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::users::service::service::UserTable;

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

use tokio::time::{self, Duration};

pub async fn user_runtime_scheduler(connection: PooledPg) {
    async fn periodic_task(connection: PooledPg) {
        let _ = UserTable::new(connection).remove_un_verified_users();
    }

    println!("test");

    let interval = Duration::from_secs(86400);
    periodic_task(connection).await;

    time::sleep(interval).await;
}
