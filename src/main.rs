pub mod api_doc;
pub mod common;
pub mod dive_chat;
pub mod dive_sites;
pub mod images;
pub mod logbook;
pub mod schema;
pub mod users;

use tracing::info;

use crate::common::env::ENV;

use api_doc::api_doc::ApiDoc;
use socketioxide::SocketIo;

use tower_http::{cors::CorsLayer, trace::TraceLayer};

use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use tokio::net::TcpListener;

use dive_chat::{
    chat_socket::chat_socket_events::{on_connect, ChatSocketState},
    kafka_chat_handler::KafkaChatHandler,
};

use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use crate::common::db;

use logbook::router::{self as logbook_routes};

use crate::common::db::ConnectionPool;
use tower_http::services::fs::ServeDir;

pub struct SharedState {
    pub connection_pool: ConnectionPool,
    pub kafka_chat_handler: KafkaChatHandler,
    pub env: ENV
}
use tower::ServiceBuilder;

async fn handler(axum::extract::State(io): axum::extract::State<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() {
    let env = ENV::new();

    let hosts = vec!["localhost:9092".to_string()];
    let kafka_chat_handler =
        KafkaChatHandler::new(hosts.clone(), String::from("dive_messages"))
            .await
            .unwrap();
    let db_url = env.database_url;
    let api_host = env.app_host;
    let app_port: u16 = env.app_port;

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 10);
    let address = SocketAddr::from((api_host, app_port));

    let (layer, io) = SocketIo::builder()
        .with_state(ChatSocketState {
            kafka_chat_handler: kafka_chat_handler.clone(),
        })
        .build_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);

    let shared_state = Arc::new(SharedState {
        connection_pool: shared_connection_pool.clone(),
        kafka_chat_handler,
        env: ENV::new(),
    });

    let app = Router::new()
        .nest_service(
            "/assets",
            axum::routing::get_service(
                ServeDir::new("assets").append_index_html_on_directories(false),
            ),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/hello", axum::routing::get(handler))
        .with_state(io)
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(logbook_routes::router::logbook_routes(shared_state.clone()))
        .merge(users::router::router::user_routes(shared_state.clone()))
        .merge(dive_chat::router::chat_sites_routes(shared_state.clone()))
        .merge(dive_sites::router::dive_sites_routes(shared_state.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let listener = TcpListener::bind(&address).await;

    println!(
        "the server listening on {}{}:{}",
        env.app_protocol,
        env.app_host,
        env.app_port
    );
    let _res = axum::serve(listener.unwrap(), app.into_make_service())
        .await
        .unwrap();
    
    common::runtime_scheduler::runtime_scheduler(
        shared_connection_pool.clone().pool.get().unwrap(),
    )
    .await;
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
