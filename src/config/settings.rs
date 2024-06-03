use std::fs;
use std::process::exit;
use config::FileFormat::Toml;
use serde::{Deserialize, Serialize};

// Define a struct to hold your application settings
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentalSettings{
    pub is_prod: String
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
pub fn load_settings() -> Settings {
    // load the env file to check what env it is

    let contents = match fs::read_to_string("config/env.toml") {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", "config/env.toml");
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let decoder = toml::Deserializer::new(&*contents);
    let env_config = EnvironmentalSettings::deserialize(decoder).unwrap();
    
    let mut filename: &str = "config/production.toml";
    
    if env_config.is_prod == "false" {
        filename = "config/development.toml";
    }

    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let decoder = toml::Deserializer::new(&*contents);
    let config = Settings::deserialize(decoder).unwrap();
    return config;
}
