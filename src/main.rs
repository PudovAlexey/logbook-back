pub mod logbook;
pub mod schema;
pub mod apiDoc;
pub mod users;
pub mod common;
pub mod images;
use tokio::time::{self, Duration};

use crate::common::env::ENV;

use apiDoc::apiDoc::ApiDoc;


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

use tower_http::services::fs::ServeDir;

async fn periodic_task() {
    // Ваша функция, которая будет вызываться каждый определенный интервал времени
    println!("Выполняется периодическая задача");
}

#[tokio::main]
async fn main() {
    let _port = ENV::new().APP_HOST;
    let db_url = ENV::new().DATABASE_URL;
    let api_host = ENV::new().APP_HOST;
    let app_port: u16 = ENV::new().APP_PORT;

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 10);
    let address = SocketAddr::from((api_host, app_port));
    let listener = TcpListener::bind(&address).await;


    let app = Router::new()
    .nest_service("/assets", axum::routing::get_service(ServeDir::new("assets")
    .append_index_html_on_directories(false)))

    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_connection_pool.clone()))
    .merge(users::router::router::user_routes(shared_connection_pool.clone()))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());

    let interval = Duration::from_secs(10);

    let periodic_task_handle = tokio::spawn(async move {
        loop {
            common::runtime_scheduler::runtime_scheduler().await;
            // // Вызываем задачу
            // periodic_task().await;

            // // Ждем до следующего интервала времени
            // time::sleep(interval).await;
        }
    });

axum::serve(listener.unwrap(), app.into_make_service()).await;
periodic_task_handle.await.expect("Failed to run periodic task");
// common::runtime_scheduler::runtime_scheduler().await;
}

// use std::fs::{
//     DirBuilder
// };

// use std::env;

// fn main() {
//     let current_dir = env::current_dir().unwrap();

//     let path = "assets";

//     let dir = current_dir.join(path);

//     println!("{:?}", dir);

// DirBuilder::new()
//     .recursive(true)
//     .create(dir).unwrap();   
// }
