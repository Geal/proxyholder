use std::sync::Arc;

use tracing_subscriber;

use crate::configuration::Configuration;

pub(crate) fn setup(configuration: &Arc<Configuration>) {
    tracing_subscriber::fmt::init();
}
