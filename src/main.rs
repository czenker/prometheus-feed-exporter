use prometheus_feed_exporter::config::app::AppConfig;
use prometheus_feed_exporter::command;
use prometheus_feed_exporter::command::Command;
use tokio;
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cli = command::parse();
    let app_config = AppConfig::load(&cli.settings_file);

    match cli.command {
        Command::Run{}=>command::run::run(&app_config).await,
        Command::Test{} => command::test::run(&app_config).await,
        Command::Debug{} => command::debug::run(&app_config).await,
    }
}
