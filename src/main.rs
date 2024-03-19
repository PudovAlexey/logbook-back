pub mod logbook;
pub mod schema;
pub mod apiDoc;
pub mod users;
pub mod common;
use crate::common::env::ENV;

use apiDoc::apiDoc::ApiDoc;




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

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 1);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;

    let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_connection_pool.clone()))
    .merge(users::router::router::user_routes(shared_connection_pool.clone()));

    axum::serve(listener, app.into_make_service()).await
}
