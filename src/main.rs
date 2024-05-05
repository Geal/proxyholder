use std::sync::Arc;

use configuration::Configuration;

mod configuration;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let configuration = Arc::new(Configuration::new()?);

    let server = network::Proxy::new(configuration).await?;

    server.run().await
}
