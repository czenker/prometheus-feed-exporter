use std::collections::HashMap;

use config::{Config, File, FileFormat};
use serde::Deserialize;

use super::common::MyCelProgram;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default="PushgatewayConfig::new")]
    pub pushgateway: PushgatewayConfig,
    pub feeds: HashMap<String, FeedConfig>,
}

#[derive(Deserialize, Debug)]
pub struct FeedConfig {
    pub uri: String,
    #[serde(default)]
    pub headers: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub excludes: Vec<MyCelProgram>,
    #[serde(default)]
    pub includes: Vec<MyCelProgram>,
    #[serde(default)]
    pub entry_id: Option<MyCelProgram>,
}

#[derive(Deserialize, Debug)]
pub struct PushgatewayConfig {
    #[serde(default)]
    pub endpoint: Option<String>,
}

impl PushgatewayConfig {
    fn new() -> PushgatewayConfig {
        // @TODO: why can serde not guess this trivial implementation?
        PushgatewayConfig {
            endpoint: None,
        }
    }
}

impl AppConfig {
    pub fn load(file_name: &String) -> AppConfig {
        let config = Config::builder()
            .add_source(File::new(file_name, FileFormat::Yaml))
            .build()
            .unwrap();

        config.try_deserialize().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let ref app_config = AppConfig::load(&String::from("tests/fixtures/appconfig/fields"));

        assert_eq!(Some("http://example.com:9000"), app_config.pushgateway.endpoint.as_deref());
        assert_eq!(1, app_config.feeds.len());
        let entry_0 = app_config.feeds.get("foobar").unwrap();
        assert_eq!("https://example.com/feed.xml", entry_0.uri);
    }

    #[test]
    fn test_load_uses_defaults() {
        let ref app_config = AppConfig::load(&String::from("tests/fixtures/appconfig/uses_defaults"));

        assert_eq!(None, app_config.pushgateway.endpoint);
        assert_eq!(1, app_config.feeds.len());
        let entry_0 = app_config.feeds.get("foobar").unwrap();
        assert_eq!("https://example.com/feed.xml", entry_0.uri);
        assert_eq!(0, entry_0.headers.len());
        assert_eq!(0, entry_0.includes.len());
        assert_eq!(0, entry_0.excludes.len());
        assert_eq!(None, entry_0.entry_id);
    }

    // @TODO: tests for cel program parsing
}