pub mod api;
pub mod auth;
pub mod database;
pub mod image_cache;
pub mod logging;
pub mod mail;
pub mod net;
pub mod tracker;
pub mod tracker_statistics_importer;
pub mod website;

use logging::Logging;
use serde::{Deserialize, Serialize};

use self::api::Api;
use self::auth::{Auth, SecretKey};
use self::database::Database;
use self::image_cache::ImageCache;
use self::mail::Mail;
use self::net::Network;
use self::tracker::{ApiToken, Tracker};
use self::tracker_statistics_importer::TrackerStatisticsImporter;
use self::website::Website;
use super::validator::{ValidationError, Validator};

/// The whole configuration for the index.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// The logging configuration.
    #[serde(default)]
    pub logging: Logging,
    /// The website customizable values.
    #[serde(default)]
    pub website: Website,
    /// The tracker configuration.
    #[serde(default)]
    pub tracker: Tracker,
    /// The network configuration.
    #[serde(default)]
    pub net: Network,
    /// The authentication configuration.
    #[serde(default)]
    pub auth: Auth,
    /// The database configuration.
    #[serde(default)]
    pub database: Database,
    /// The SMTP configuration.
    #[serde(default)]
    pub mail: Mail,
    /// The image proxy cache configuration.
    #[serde(default)]
    pub image_cache: ImageCache,
    /// The API configuration.
    #[serde(default)]
    pub api: Api,
    /// The tracker statistics importer job configuration.
    #[serde(default)]
    pub tracker_statistics_importer: TrackerStatisticsImporter,
}

impl Settings {
    pub fn remove_secrets(&mut self) {
        self.tracker.token = ApiToken::new("***");
        if let Some(_password) = self.database.connect_url.password() {
            let _ = self.database.connect_url.set_password(Some("***"));
        }
        "***".clone_into(&mut self.mail.password);
        self.auth.secret_key = SecretKey::new("***");
    }
}

impl Validator for Settings {
    fn validate(&self) -> Result<(), ValidationError> {
        self.tracker.validate()
    }
}
