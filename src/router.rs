use std::sync::Arc;

use hyper::Request;

use crate::{configuration::Configuration, service::Service};

pub(crate) struct Router {}

impl Router {
    pub(crate) async fn new(configuration: &Arc<Configuration>) -> Self {
        Self {}
    }

    pub(crate) async fn router<Body>(&self, req: Request<Body>) -> Option<Arc<Service>> {
        Some(Arc::new(Service::new()))
    }
}
