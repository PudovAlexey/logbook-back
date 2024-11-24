use dotenvy::dotenv;
use std::env;

pub fn load_env_variable(variable_name: &str) -> String {
    dotenv().ok();
    env::var(variable_name).expect(&format!("{} must be set in .env file", variable_name))
}
