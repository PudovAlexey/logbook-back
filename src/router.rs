pub mod user;
use crate::api_doc;
use crate::dive_sites;
use crate::logbook;
use crate::router::user::UserApiDoc;
use crate::users as old_users;
use axum::Router;
use tower_http::services::fs::ServeDir;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
// use api_doc::api_doc::ApiDoc;
use crate::SharedStateType;
use logbook::router::{self as logbook_routes};

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(),
    // modifiers(&SecurityAddon),
    info(title = "Ordinotes API", description = "API description"),
    tags(
        (name = "user", description = "Users endpoints"),
    )
)]
pub struct ApiDoc;

pub fn create_router(shared_state: SharedStateType) -> Router {
    let mut openapi = ApiDoc::openapi();
    openapi.merge(UserApiDoc::openapi());

    Router::new()
        .nest_service(
            "/assets",
            axum::routing::get_service(
                ServeDir::new("assets").append_index_html_on_directories(false),
            ),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi.clone()))
        // .merge()
        .merge(logbook_routes::router::logbook_routes(shared_state.clone()))
        .merge(old_users::router::router::user_routes(shared_state.clone()))
        .merge(dive_sites::router::dive_sites_routes(shared_state.clone()))
        .merge(user::user_routes_v2(shared_state.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
