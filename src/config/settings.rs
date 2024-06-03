use serde::{Deserialize, Serialize};

// Define a struct to hold your application settings
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentalSettings{
    pub is_prod: bool
}

// Define a struct to hold database connection settings
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    // Add more database-specific settings as needed
}

// Define a struct to hold server configuration settings
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    // Add more server-specific settings as needed
}

// Implement a function to load settings from a configuration file
pub fn load_settings() -> Result<Settings, config::ConfigError> {
    let mut envConfig = config::Config::default();
    envConfig.merge(config::File::with_name("config/env"))?;
    let env:EnvironmentalSettings = envConfig.try_into().unwrap();
    
    
    let mut config = config::Config::default();
    config.merge(config::File::with_name("config/default"))?;
    if env.is_prod {
        config.merge(config::File::with_name("config/production").required(false))?;
    } else {
        config.merge(config::File::with_name("config/development").required(false))?;
    }
    config.try_into()
}
