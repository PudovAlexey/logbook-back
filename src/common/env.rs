use std::net::Ipv4Addr;

use crate::common::load_env_variable::load_env_variable;

pub struct ENV {
   pub APP_PROTOCOL: String, 
   pub APP_HOST: Ipv4Addr,
   pub DATABASE_URL: String,
   pub JWT_ACCESS_SECRET: String,
   pub JWT_REFRESH_SECRET: String,
   pub JWT_ACCESS_EXPIRED_IN: i64,
   pub JWT_REFRESH_EXPIRED_IN: i64,
   pub SMTP_USERNAME: String,
   pub SMTP_PASSWORD: String,
   pub SMTP_TRANSPORT: String,
   pub REDIS_PORT: String,
   pub APP_PORT: u16,
}

impl ENV {
    pub fn new() -> Self {
        Self {
            APP_PORT: load_env_variable("APP_PORT").parse().unwrap(),
            APP_HOST: load_env_variable("APP_HOST").parse().unwrap(),
            DATABASE_URL: load_env_variable("DATABASE_URL"),
            JWT_REFRESH_SECRET: load_env_variable("JWT_REFRESH_SECRET"),
            JWT_ACCESS_SECRET: load_env_variable("JWT_ACCESS_SECRET"),
            JWT_ACCESS_EXPIRED_IN: load_env_variable("JWT_ACCESS_EXPIRED_IN").parse().unwrap(),
            JWT_REFRESH_EXPIRED_IN: load_env_variable("JWT_REFRESH_EXPIRED_IN").parse().unwrap(),
            SMTP_USERNAME: load_env_variable("SMTP_USERNAME"),
            SMTP_PASSWORD: load_env_variable("SMTP_PASSWORD"),
            SMTP_TRANSPORT: load_env_variable("SMTP_TRANSPORT"),
            REDIS_PORT: load_env_variable("REDIS_PORT"),
            APP_PROTOCOL: load_env_variable("APP_PROTOCOL"),
        }
    }
}

