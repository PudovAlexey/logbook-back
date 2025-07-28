pub mod api_doc;
pub mod common;
pub mod dive_sites;
pub mod error;
pub mod images;
pub mod logbook;
pub mod otel;
pub mod router;
pub mod schema;
pub mod service;
pub mod users;
pub mod utils;

use crate::{
    common::{db::ConnectionPool, env::ENV, redis::Redis},
    otel::{Metrics, MetricsSubscriber},
    router::create_router,
};

use tokio::net::TcpListener;

use std::{net::SocketAddr, sync::Arc};

use crate::common::db;

pub struct SharedState {
    pub db_pool: Arc<ConnectionPool>,
    pub redis: Arc<Redis>,
    pub metrics: Arc<Metrics>,
    pub env: Arc<ENV>,
}

pub type SharedStateType = Arc<SharedState>;

#[tokio::main]
async fn main() {
    let env = Arc::new(ENV::new());

    let main_env = Arc::clone(&env);

    let ENV {
        database_url,
        app_host,
        app_port,
        ..
    } = &*main_env;

    let state_env = Arc::clone(&env);

    let shared_connection_pool = Arc::new(db::create_shared_connection_pool(
        database_url.to_owned(),
        10,
    ));
    let address = SocketAddr::from((app_host.to_owned(), app_port.to_owned()));
    let listener = TcpListener::bind(&address).await;
    let redis = Arc::new(Redis::new());
    let mut metrics_subscriber = MetricsSubscriber::new();

    let shared_state = Arc::new(SharedState {
        db_pool: shared_connection_pool.clone(),
        redis: redis.clone(),
        metrics: metrics_subscriber.metrics.clone(),
        env: state_env.clone(),
    });

    let app = create_router(shared_state.clone());

    println!("the servere running on http://{:?}", address);

    tokio::spawn(async move {
        metrics_subscriber.run_metrics_server().await;
    });

    tokio::spawn(async move {
        common::runtime_scheduler::runtime_scheduler(
            shared_connection_pool.clone().pool.get().unwrap(),
        )
        .await
    });

    let _res = axum::serve(listener.unwrap(), app.into_make_service()).await;
}
