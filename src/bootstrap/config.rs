//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_INDEX_`.

// Environment variables

use crate::config::{Configuration, Info};

/// The whole `index.toml` file content. It has priority over the config file.
/// Even if the file is not on the default path.
const ENV_VAR_CONFIG: &str = "TORRUST_INDEX_CONFIG";

/// Token needed to communicate with the Torrust Tracker
const ENV_VAR_API_ADMIN_TOKEN: &str = "TORRUST_INDEX_TRACKER_API_TOKEN";

/// The `index.toml` file location.
pub const ENV_VAR_PATH_CONFIG: &str = "TORRUST_INDEX_PATH_CONFIG";

// Default values
pub const DEFAULT_PATH_CONFIG: &str = "./share/default/config/index.development.sqlite3.toml";

/// If present, CORS will be permissive.
pub const ENV_VAR_CORS_PERMISSIVE: &str = "TORRUST_INDEX_BACK_CORS_PERMISSIVE";

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `index.toml`.
/// 2. Environment variable: `TORRUST_INDEX_CONFIG`. The variable contains the same contents as the `index.toml` file.
///
/// Environment variable has priority over the config file.
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-index-configuration) for the configuration options.
///
/// # Panics
///
/// Will panic if it can't load the configuration from either
/// `./index.toml` file or the env var `TORRUST_INDEX_CONFIG`.
#[must_use]
pub fn initialize_configuration() -> Configuration {
    let info = Info::new(
        ENV_VAR_CONFIG.to_string(),
        ENV_VAR_PATH_CONFIG.to_string(),
        DEFAULT_PATH_CONFIG.to_string(),
        ENV_VAR_API_ADMIN_TOKEN.to_string(),
    )
    .unwrap();

    Configuration::load(&info).unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_load_with_default_config() {
        use crate::bootstrap::config::initialize_configuration;

        drop(initialize_configuration());
    }
}
