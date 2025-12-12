use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub optimization: OptimizationConfig,
    pub learning: LearningConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptimizationConfig {
    pub svd_keep_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningConfig {
    pub learning_rate: f64,
    pub momentum: f64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            // Set defaults
            .set_default("server.log_level", "info")?
            .set_default("optimization.svd_keep_ratio", 0.9)?
            .set_default("learning.learning_rate", 0.01)?
            .set_default("learning.momentum", 0.9)?
            // Layer on environment variables (with prefix TENSOR__)
            .add_source(Environment::with_prefix("TENSOR").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}
