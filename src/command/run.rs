use std::cmp::max;

use crate::config::app::{AppConfig, FeedConfig, PushgatewayConfig};
use crate::utils::feed::{self, get_cel_context_for, get_feed};
use reqwest;
use log::{info, debug};
use cel_interpreter::Value;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use metrics::{counter, describe_counter, gauge, describe_gauge};

pub async fn run(app_config: &AppConfig) {
    let prometheus_handle = init_metrics();

    for (id, feed_config) in &app_config.feeds {
        analyze(&id, &feed_config).await;
    }

    let prometheus_string = prometheus_handle.render();

    match app_config.pushgateway.endpoint {
        Some(_) => push_metrics(&app_config.pushgateway, &prometheus_string).await,
        None => println!("{}", prometheus_string),
    }
}

fn init_metrics() -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    let recorder = builder
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    //describe_gauge!("feeds_total", "number of feeds configured");
    //describe_gauge!("feeds_error", "number of feeds that ran into an error that might lead to a wrong report");

    describe_counter!("feed_info", "information of a feed");
    describe_gauge!("feed_updated_at", "the last significant change of this feed as REPORTED BY THE CREATOR");
    describe_gauge!("feed_pulled_at", "the last time this feed was pulled");
    describe_gauge!("feed_entries", "number of entries per feed");
    describe_gauge!("feed_significant_entries", "number of entries per feed that have not been filtered out by the given rules");
    describe_gauge!("feed_last_significant_entry_at", "publication time of the last entry that has not been filtered out by the given rules");
    describe_gauge!("feed_significant_entry_info", "a significant entry");

    recorder
}

async fn push_metrics(config: &PushgatewayConfig, body: &String) {
    // Implementation of metrics-exporter-prometheus for push-gateway is an asynchronous, delayed and repeated push.
    // This is not what we want as we only need to push metrics once this job is done and we need to wait
    // for it before ending the program. This is not possible with its current implementation, so we
    // implement the logic ourselves
    // @see https://docs.rs/metrics-exporter-prometheus/0.13.1/src/metrics_exporter_prometheus/builder.rs.html#465-523

    let ref endpoint = config.endpoint.clone().expect("push_metrics must not be called with an empty endpoint");

    let client = reqwest::Client::new();
    let res = client.put(endpoint)
        .body(body.clone())
        .send()
        .await;
    res.expect("Metrics should be pushed to push gateway");
    // @TODO: error handling
    info!("Pushed metrics to {}.", endpoint);
}

async fn analyze(id: &String, config: &FeedConfig) {
    debug!("Analyzing {}", id);
    let feed = get_feed(config).await;
    let now = crate::utils::time::now();

    let title = feed.title.map(|t| t.content).unwrap_or(String::from(""));
    let feed_updated_timestamp = feed.updated.map(|t| t.timestamp()).unwrap_or_default();
    let entry_num = feed.entries.len();

    counter!("feed_info", "title" => title.clone(), "feed" => id.clone()).increment(1);
    gauge!("feed_updated_at", "feed" => id.clone()).set(feed_updated_timestamp as f64);
    gauge!("feed_pulled_at", "feed" => id.clone()).set(now.timestamp() as f64);
    gauge!("feed_entries", "feed" => id.clone()).set(entry_num as f64);

    let mut significant_entry_num = 0;
    let mut last_significant_entry_timestamp = 0;

    let program = config.entry_id.as_ref();

    let entries = feed.entries.iter().collect();

    let filtered_entries = feed::filter(&entries, &config.includes.iter().collect(), &config.excludes.iter().collect(), &now);

    for entry in filtered_entries {
        let title = entry.title.clone().map(|t| t.content).unwrap(); // @TODO: resolve hacky borrow circumvention
        let context = get_cel_context_for(&entry, &now); // @TODO: don't generate context again

        let entry_id = {
            match program {
                Some(ref p) => match p.execute(&context).unwrap() {
                    Value::String(id) => id.to_string(),
                    _ => entry.id.clone(),
                },
                _ => entry.id.clone(),
            }
        };

        let timestamp = entry.updated.map(|t| t.timestamp()).unwrap_or_default();
        last_significant_entry_timestamp = max(last_significant_entry_timestamp, timestamp);
        significant_entry_num += 1;
        debug!("{:?} {:?}", title, entry_id);
        gauge!("feed_significant_entry_info", "feed" => id.clone(), "entry" => entry_id.clone()).set(1.);
    }

    gauge!("feed_significant_entries", "feed" => id.clone()).set(significant_entry_num as f64);
    gauge!("feed_last_significant_entry_at", "feed" => id.clone()).set(last_significant_entry_timestamp as f64);
}
