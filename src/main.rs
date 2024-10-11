pub mod logbook;
pub mod schema;
pub mod api_doc;
pub mod users;
pub mod common;
pub mod dive_sites;
pub mod images;
pub mod dive_chat;

use tracing::{info};

use crate::common::env::ENV;

use api_doc::api_doc::ApiDoc;
use socketioxide::SocketIo;

use tower_http::{cors::CorsLayer, trace::TraceLayer};




use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
    
use tokio::net::TcpListener;

use dive_chat::{chat_consumer::ChatConsumer, chat_producer::ChatProducer, chat_socket::chat_socket_events::{on_connect, ChatSocketState}};


use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use crate:: common::db;

use logbook::router::{self as logbook_routes};

use tower_http::services::fs::ServeDir;
use crate::common::db::ConnectionPool;

pub struct SharedState {
   pub connection_pool: ConnectionPool,
   pub chat_producer: ChatProducer,
}
use tower::ServiceBuilder;

async fn handler(axum::extract::State(io): axum::extract::State<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() {
    let hosts = vec![ "localhost:9092".to_string() ];
    let mut chat_producer = ChatProducer::new( hosts.clone() );
    let mut chat_consumer = ChatConsumer::new(hosts.clone(), String::from("dive_messages"));
    let db_url = ENV::new().database_url;
    let api_host = ENV::new().app_host;
    let app_port: u16 = ENV::new().app_port;

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 10);
    let address = SocketAddr::from((api_host, app_port));

    let (layer, io) = SocketIo::builder().with_state(ChatSocketState {
        chat_consumer,
    }).build_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);

    let shared_state = Arc::new(SharedState {
        connection_pool: shared_connection_pool.clone(),
        chat_producer,
    });


    let app = Router::new()
    .nest_service("/assets", axum::routing::get_service(ServeDir::new("assets")
    .append_index_html_on_directories(false)))
    
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .route("/hello", axum::routing::get(handler))
    .with_state(io)
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_connection_pool.clone()))
    .merge(users::router::router::user_routes(shared_connection_pool.clone()))
    .merge(dive_chat::router::chat_sites_routes(shared_state))
    .merge(dive_sites::router::dive_sites_routes(shared_connection_pool.clone()))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http())
    .layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
    );

let listener = TcpListener::bind(&address).await;

println!("the server listening on {}{}:{}", ENV::new().app_protocol, ENV::new().app_host, ENV::new().app_port);
let _res = axum::serve(listener.unwrap(), app.into_make_service()).await.unwrap();
common::runtime_scheduler::runtime_scheduler(shared_connection_pool.clone().pool.get().unwrap()).await;
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tracing::subscriber::set_global_default(FmtSubscriber::default())?;

//     let (layer, io) = SocketIo::builder().with_state(Vec::from([1, 2, 3])).build_layer();

//     io.ns("/", on_connect);

//     let app = axum::Router::new()
//     .route("/", axum::routing::get(|| async { "Hello, World!" }))
//     .route("/hello", axum::routing::get(handler))
//     .with_state(io)
//     .layer(
//         ServiceBuilder::new()
//             .layer(CorsLayer::permissive())
//             .layer(layer),
//     );

//     let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
//     axum::serve(listener, app).await.unwrap();

//     todo!()
// }
