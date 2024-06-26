use feed_rs::{model::Entry, parser};

use crate::{config::app::AppConfig, utils::feed::{get_cel_context_for, debug_cel_context}};


pub async fn run(app_config: &AppConfig) {
    let config = app_config.feeds.get("helm-releases").unwrap();

    let body = reqwest::get(&config.uri)
        .await
        .expect("Foobar")
        .text()
        .await
        .expect("Foobar");

    let now = crate::utils::time::now();
    let feed = parser::parse(body.as_bytes()).expect("Foobar");

    let entries :Vec<&Entry> = feed.entries.iter().collect();

    //let filtered_entries = feed::filter(&entries, &config.includes.iter().collect(), &config.excludes.iter().collect(), &now);

    for entry in entries {
        let context = get_cel_context_for(&entry, &now); // @TODO: don't generate context again
        debug_cel_context(&context);

    }
}
