use std::fs;
use std::ops::Add;

use crate::config::app::AppConfig;
use crate::config::common::MyCelProgram;
use crate::config::test::{TestCaseConfig, TestConfig};
use crate::utils::feed::filter;
use feed_rs::parser;

use colored::Colorize;

pub async fn run(app_config: &AppConfig, file_selector: &String) {

    let test_config = TestConfig::load(file_selector);
    let mut test_result = TestResult::new();

    for case in test_config.cases.iter() {
        test_result = test_result + run_case(app_config, &case);
    }

    println!("{} success, {} failure", test_result.success, test_result.failure);
    if test_result.failure > 0 {
        std::process::exit(-1);
    }
}

fn print_test_success(case: &String, it: &String) {
    println!(
        "{} {} {}",
        "✓".green(),
        format!("[{}]", case).dimmed(),
        it
    );
}

fn print_test_failure(case: &String, it: &String) {
    println!(
        "{} {} {}",
        "✗".red(),
        format!("[{}]", case).dimmed(),
        it
    );
}

struct TestResult {
    success: u32,
    failure: u32,
}

impl TestResult {
    fn new() -> TestResult {
        return TestResult{
            success: 0,
            failure: 0,
        }
    }

    fn success() -> TestResult {
        return TestResult{
            success: 1,
            failure: 0,
        }
    }

    fn failure() -> TestResult {
        return TestResult{
            success: 0,
            failure: 1,
        }
    }
}

impl Add for TestResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            success: self.success + other.success,
            failure: self.failure + other.failure,
        }
    }
}

fn run_case(app_config: &AppConfig, test_config: &TestCaseConfig) -> TestResult {
    let feed_id = test_config.setup.feed_name.clone();
    let feed_config = app_config.feeds.get(&feed_id).expect(&format!("expected to find configuration for {}", feed_id));
    let contents = fs::read_to_string(test_config.setup.feed_file.clone()).expect("Should have been able to read the file");

    let feed = parser::parse(contents.as_bytes()).expect("Foobar");

    let mut test_result = TestResult::new();

    let now = match &test_config.setup.now {
        Some(time_string) => crate::utils::time::from_string(&time_string),
        None => crate::utils::time::now(),
    };

    let entries = feed.entries.iter().collect();
    let filtered_entries = filter(&entries, &feed_config.includes.iter().collect(), &feed_config.excludes.iter().collect(), &now);

    for test in &test_config.tests {
        let mut includes: Vec<&MyCelProgram> = Vec::new();
        includes.push(&test.query); // @TODO: Shouldn't there be a literal for this?
        let excludes: Vec<&MyCelProgram> = Vec::new();
        let result = filter(&filtered_entries, &includes, &excludes, &now);
        let found = result.len() > 0;

        if found ^ test.expected {
            test_result = test_result + TestResult::failure();
            print_test_failure(&test_config.name, &test.it);
        } else {
            test_result = test_result + TestResult::success();
            print_test_success(&test_config.name, &test.it);
        }
    }

    test_result
}