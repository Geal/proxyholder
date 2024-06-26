use std::sync::Arc;

use configuration::Configuration;

mod configuration;
mod network;
mod router;
mod service;
mod telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let configuration = Arc::new(Configuration::new()?);
    telemetry::setup(&configuration);

    let server = network::Proxy::new(configuration).await?;

    server.run().await
}
