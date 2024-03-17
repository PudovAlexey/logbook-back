use crate::common::load_env_variable::load_env_variable;

pub struct ENV {
   pub APP_HOST: String,
   pub DATABASE_URL: String,
   pub JWT_SECRET: String,
   pub JWT_EXPIRED_IN: String,
   pub JWT_MAX_AGE: String,
   pub SMTP_USERNAME: String,
   pub SMTP_PASSWORD: String,
   pub SMTP_TRANSPORT: String,
   pub REDIS_PORT: String,
}

impl ENV {
    pub fn new() -> Self {
        Self {
            APP_HOST: load_env_variable("APP_HOST"),
            DATABASE_URL: load_env_variable("DATABASE_URL"),
            JWT_SECRET: load_env_variable("JWT_SECRET"),
            JWT_EXPIRED_IN: load_env_variable("JWT_EXPIRED_IN"),
            JWT_MAX_AGE: load_env_variable("JWT_MAX_AGE"),
            SMTP_USERNAME: load_env_variable("SMTP_USERNAME"),
            SMTP_PASSWORD: load_env_variable("SMTP_PASSWORD"),
            SMTP_TRANSPORT: load_env_variable("SMTP_TRANSPORT"),
            REDIS_PORT: load_env_variable("REDIS_PORT"),
        }
    }
}

