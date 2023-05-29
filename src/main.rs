mod domain;

use email_api::configuration::get_configuration;
use email_api::startup::Application;
use email_api::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
