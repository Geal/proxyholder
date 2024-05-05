use std::sync::Arc;

use crate::configuration::Configuration;

pub(crate) struct Proxy {}

impl Proxy {
    pub(crate) async fn new(
        configuration: Arc<Configuration>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Proxy {})
    }

    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
