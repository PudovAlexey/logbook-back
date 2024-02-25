pub mod apiDoc {
    use utoipa::{
        openapi, 
        OpenApi, 
        Modify,
        openapi::security::{SecurityScheme, ApiKey, ApiKeyValue}
    };

    use crate::logbook;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            logbook::router::router::get_logbook_list,
        ),
        components(
            schemas(logbook::model::LogInfo)
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "loginfo", description = "Log info schema")
        )
    )]
    pub struct ApiDoc;

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
}