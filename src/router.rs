pub mod user;
use crate::users as old_users;
use crate::logbook;
use crate::dive_sites;
use crate::api_doc;
use axum::Router;
use tower_http::services::fs::ServeDir;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use api_doc::api_doc::ApiDoc;
use utoipa::OpenApi;
use logbook::router::{self as logbook_routes};
use crate::SharedStateType;


pub fn create_router(shared_state: SharedStateType) -> Router {
    Router::new()
        .nest_service(
            "/assets",
            axum::routing::get_service(
                ServeDir::new("assets").append_index_html_on_directories(false),
            ),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(logbook_routes::router::logbook_routes(shared_state.clone()))
        .merge(old_users::router::router::user_routes(shared_state.clone()))
        .merge(dive_sites::router::dive_sites_routes(shared_state.clone()))
        .merge(user::user_routes_v2(shared_state.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
