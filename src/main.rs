pub mod logbook;
pub mod schema;
pub mod apiDoc;
pub mod users;
pub mod common;
use crate::common::env::ENV;

use apiDoc::apiDoc::ApiDoc;

use http::HeaderValue;
use tower_http::{cors::CorsLayer, trace::TraceLayer};




use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
    
use tokio::net::TcpListener;


use std::{
    net::{Ipv4Addr, SocketAddr},
};

use axum::{Router};

use crate:: {
    common::db
};

use logbook::router::{self as logbook_routes};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _port = ENV::new().APP_HOST;
    let db_url = ENV::new().DATABASE_URL;
    let api_host = ENV::new().APP_HOST;
    let app_port: u16 = ENV::new().APP_PORT;

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 1);
    let address = SocketAddr::from((api_host, app_port));
    let listener = TcpListener::bind(&address).await?;

    let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_connection_pool.clone()))
    .merge(users::router::router::user_routes(shared_connection_pool.clone()))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
    // .layer(CorsLayer::new().allow_origin(HeaderValue::from_static("http://localhost:8081")));
    // .layer(CorsLayer::new().allow_origin("http://localhost:8081/".parse::<HeaderValue>().unwrap()));

    axum::serve(listener, app.into_make_service()).await
}
