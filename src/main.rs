pub mod common;
pub mod logbook;
pub mod schema;
pub mod apiDoc;
pub mod users;
use crate::common::env::ENV;

use apiDoc::apiDoc::ApiDoc;

use common::{
    mailer::Mailer
};

use schema::loginfo::user_id;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
    
use tokio::net::TcpListener;
use common::{load_env_variable::load_env_variable};

use std::{
    net::{Ipv4Addr, SocketAddr},
};

use axum::{Router};

use crate:: {
    common::env,
    common::db
};

use logbook::router::{self as logbook_routes};

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let port = load_env_variable(&ENV::new().APP_HOST);
//     let db_url = load_env_variable(&ENV::new().DATABASE_URL);

//     let shared_connection_pool = db::create_shared_connection_pool(db_url, 1);
//     let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
//     let listener = TcpListener::bind(&address).await?;

//     let app = Router::new()
//     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
//     .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
//     .merge(logbook_routes::router::logbook_routes(shared_connection_pool.clone()))
//     .merge(users::router::router::user_routes(shared_connection_pool.clone()));

//     axum::serve(listener, app.into_make_service()).await
// }

// fn main() {
//     let mailer =  Mailer::new(Mailer {
//         to: "hajecig739@hidelux.com".to_string(),
//         subject: "New subject".to_string(),
//         body: "New email body".to_string(),
//     });

//     mailer.send();
//     // println!("{}", &ENV::new().SMTP_PASSWORD)
// }
