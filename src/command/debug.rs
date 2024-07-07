use feed_rs::model::Entry;

use crate::{config::app::AppConfig, utils::feed::{debug_cel_context, get_cel_context_for, get_feed}};

pub async fn run(app_config: &AppConfig, feed: &String) {
    let config = app_config.feeds
        .get(feed)
        .expect(format!("A feed with name {} does not exist", feed).as_str());

    let feed = get_feed(config).await;
    let now = crate::utils::time::now();

    let entries :Vec<&Entry> = feed.entries.iter().collect();

    //let filtered_entries = feed::filter(&entries, &config.includes.iter().collect(), &config.excludes.iter().collect(), &now);

    for entry in entries {
        let context = get_cel_context_for(&entry, &now); // @TODO: don't generate context again
        debug_cel_context(&context);
    }
}
