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

use opentelemetry::metrics::Meter;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use tokio::net::TcpListener;

use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use crate::common::db;

use logbook::router::{self as logbook_routes};

use tower_http::services::fs::ServeDir;

pub struct SharedState {
    pub db_pool: Arc<ConnectionPool>,
    pub redis: Arc<Redis>,
    pub metrics: Arc<Metrics>,
}

pub type SharedStateType = Arc<SharedState>;

#[tokio::main]
async fn main() {
    // new_telemetry();
    let db_url = ENV::new().database_url;
    let api_host = ENV::new().app_host;
    let app_port: u16 = ENV::new().app_port;

    let shared_connection_pool = Arc::new(db::create_shared_connection_pool(db_url, 10));
    let address = SocketAddr::from((api_host, app_port));
    let listener = TcpListener::bind(&address).await;
    let redis = Arc::new(Redis::new());
    let mut metrics_subscriber = MetricsSubscriber::new();

    let shared_state = Arc::new(SharedState {
        db_pool: shared_connection_pool.clone(),
        redis: redis.clone(),
        metrics: metrics_subscriber.metrics.clone(),
    });

    let app = create_router(shared_state.clone());

    println!(
        "the server listening on {}{}:{}",
        ENV::new().app_protocol,
        ENV::new().app_host,
        ENV::new().app_port
    );

    tokio::spawn(async move {
        metrics_subscriber.run_metrics_server().await;
        // new_telemetry().await;
    });
    // println!("the server is down");
    let _res = axum::serve(listener.unwrap(), app.into_make_service())
        .await
        .unwrap();

    common::runtime_scheduler::runtime_scheduler(
        shared_connection_pool.clone().pool.get().unwrap(),
    )
    .await;
}
