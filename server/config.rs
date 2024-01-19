use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_port: u16,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        // Initialize the configuration reader
        let mut config = config::Config::new();

        // Merge environment variables
        config.merge(config::Environment::new().separator("__"))?;

        // Set default values for the configuration
        config.set_default("server_port", 3030)?;
        config.set_default("redis_url", "redis://127.0.0.1/")?;

        // Try to convert the configuration values into our Config struct
        config.try_into()
    }
}

// Helper function to get environment variables with a fallback
fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
