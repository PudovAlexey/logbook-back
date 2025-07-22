pub mod logbook;
pub mod schema;
pub mod api_doc;
pub mod users;
pub mod common;
pub mod dive_sites;
pub mod images;

use crate::common::{db::ConnectionPool, env::ENV, redis::Redis};

use api_doc::api_doc::ApiDoc;


use tower_http::{cors::CorsLayer, trace::TraceLayer};




use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
    
use tokio::net::TcpListener;


use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use crate:: common::db;

use logbook::router::{self as logbook_routes};

use tower_http::services::fs::ServeDir;

pub struct SharedState {
    pub db_pool: Arc<ConnectionPool>,
    pub redis: Arc<Redis>,
}

pub type SharedStateType = Arc<SharedState>;

#[tokio::main]
async fn main() {
    let db_url = ENV::new().database_url;
    let api_host = ENV::new().app_host;
    let app_port: u16 = ENV::new().app_port;

    let shared_connection_pool = Arc::new(db::create_shared_connection_pool(db_url, 10));
    let address = SocketAddr::from((api_host, app_port));
    let listener = TcpListener::bind(&address).await;
    let redis = Arc::new(Redis::new());

    let shared_state = Arc::new(SharedState {
     db_pool:  shared_connection_pool.clone(),
     redis: redis.clone(),
    }
    );

    // let shared_state = 


    let app = Router::new()
    .nest_service("/assets", axum::routing::get_service(ServeDir::new("assets")
    .append_index_html_on_directories(false)))

    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_state.clone()))
    .merge(users::router::router::user_routes(shared_state.clone()))
    .merge(dive_sites::router::dive_sites_routes(shared_state.clone()))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());

println!("the server listening on {}{}:{}", ENV::new().app_protocol, ENV::new().app_host, ENV::new().app_port);
let _res = axum::serve(listener.unwrap(), app.into_make_service()).await.unwrap();
common::runtime_scheduler::runtime_scheduler(shared_connection_pool.clone().pool.get().unwrap()).await;
}
