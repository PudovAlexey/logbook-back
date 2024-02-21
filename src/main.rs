pub mod common;
pub mod logbook;
pub mod schema;

// use axum::{
    //     Error,
    //     Router
    // };
    
    // pub mod common;
    // pub mod logbook;

use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
    
use tokio::net::TcpListener;
use common::{load_env_variable::load_env_variable};

use std::{
    net::{Ipv4Addr, SocketAddr},
};

use axum::{Router};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate:: {
    common::env,
    common::db
};

use logbook::router::{self as logbook_routes, router::logbook_routes};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

use logbook_routes::router::get_logbook_list;

#[derive(OpenApi)]
#[openapi(
    paths(
        logbook_routes::router::get_logbook_list,
    ),
    components(
        schemas(logbook_routes::router::Todo, logbook_routes::router::TodoError)
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = load_env_variable(env::APP_HOST);
    let db_url = load_env_variable(env::DATABASE_URL);

    let shared_connection_pool = db::create_shared_connection_pool(db_url, 1);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;

    let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(logbook_routes::router::logbook_routes(shared_connection_pool));
    // .merge(logbook_routes::router::logbook_routes(shared_connection_pool));

    axum::serve(listener, app.into_make_service()).await
}
