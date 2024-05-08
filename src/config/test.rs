use config::{Config, File, FileFormat};
use serde::Deserialize;

use super::common::MyCelProgram;

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub cases: Vec<TestCaseConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TestCaseConfig {
    pub name: String,
    pub setup: TestSetupConfig,
    #[serde(default)]
    pub tests: Vec<SpecConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TestSetupConfig {
    #[serde(rename = "feedFile")]
    pub feed_file: String,
    #[serde(rename = "feedName")]
    pub feed_name: String,
    pub now: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SpecConfig {
    pub it: String,
    pub query: MyCelProgram,
    pub expected: bool,
}

impl TestConfig {
    pub fn load(file_name: &String) -> TestConfig {
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
        let ref test_config = TestConfig::load(&String::from("tests/fixtures/testconfig/fields"));

        assert_eq!(1, test_config.cases.len());
        assert_eq!("some-feed", test_config.cases[0].name);
        assert_eq!("./tests/feed.xml", test_config.cases[0].setup.feed_file);
        assert_eq!("istio-news", test_config.cases[0].setup.feed_name);
        assert_eq!(Some(String::from("2023-12-31T12:34:56Z")), test_config.cases[0].setup.now);
        assert_eq!(1, test_config.cases[0].tests.len());
        assert_eq!(String::from("should find the entry for SEC-1234"), test_config.cases[0].tests[0].it);
        assert_eq!(true, test_config.cases[0].tests[0].expected);
    }

    // @TODO: tests for cel program parsing
}