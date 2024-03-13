pub mod apiDoc {
    use utoipa::{
        OpenApi, 
        Modify,
        openapi::security::{SecurityScheme, ApiKey, ApiKeyValue}
    };

    use crate::logbook;
    use crate::users;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            logbook::router::router::get_logbook_list,
            logbook::router::router::get_logbook_by_id,
            logbook::router::router::update_loginfo_handler,
            logbook::router::router::create_loginfo_handler,
            users::router::router::create_user_handler,
            users::router::router::verify_user_handler,
            users::router::router::login_user_handler,
        ),
        components(
            schemas(logbook::model::LogInfo),
            schemas(logbook::model::UpdateLogInfo),
            schemas(logbook::model::CreateLogInfo),
            schemas(users::model::CreateUserHandlerQUERY),
            schemas(users::model::LoginUser),
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