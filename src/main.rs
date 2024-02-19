pub mod common;
pub mod logbook;

// use axum::{
    //     Error,
    //     Router
    // };
    
    // pub mod common;
    // pub mod logbook;
    
use tokio::net::TcpListener;
use common::{db::create_shared_connection_pool, load_env_variable::load_env_variable};

use std::{
    net::{Ipv4Addr, SocketAddr},
};

use axum::{Error, Router};

use crate:: {
    common::env,
    common::load_env_variable,
    common::db,
    logbook::router
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = load_env_variable(env::APP_HOST);
    let db_url = load_env_variable(env::DATABASE_URL);

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 1);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port.parse().unwrap()));
    let listener = TcpListener::bind(&address).await?;

    let app = Router::new()
    .merge(router::router::logbook_routes(shared_connection_pool));

    axum::serve(listener, app.into_make_service()).await
}
