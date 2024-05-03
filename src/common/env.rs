use std::net::Ipv4Addr;

use crate::common::load_env_variable::load_env_variable;

pub struct ENV {
   pub app_protocol: String, 
   pub app_host: Ipv4Addr,
   pub database_url: String,
   pub jwt_access_secret: String,
   pub jwt_refresh_secret: String,
   pub jwt_access_expired_in: i64,
   pub jwt_refresh_expired_in: i64,
   pub smtp_username: String,
   pub smtp_password: String,
   pub smtp_transport: String,
   pub redis_port: String,
   pub app_port: u16,
}

impl ENV {
    pub fn new() -> Self {
        Self {
            app_port: load_env_variable("APP_PORT").parse().unwrap(),
            app_host: load_env_variable("APP_HOST").parse().unwrap(),
            database_url: load_env_variable("DATABASE_URL"),
            jwt_refresh_secret: load_env_variable("JWT_REFRESH_SECRET"),
            jwt_access_secret: load_env_variable("JWT_ACCESS_SECRET"),
            jwt_access_expired_in: load_env_variable("JWT_ACCESS_EXPIRED_IN").parse().unwrap(),
            jwt_refresh_expired_in: load_env_variable("JWT_REFRESH_EXPIRED_IN").parse().unwrap(),
            smtp_username: load_env_variable("SMTP_USERNAME"),
            smtp_password: load_env_variable("SMTP_PASSWORD"),
            smtp_transport: load_env_variable("SMTP_TRANSPORT"),
            redis_port: load_env_variable("REDIS_PORT"),
            app_protocol: load_env_variable("APP_PROTOCOL"),
        }
    }
}

