pub(crate) struct Service {
    uri: hyper::Uri,
}

impl Service {
    pub(crate) fn new() -> Self {
        Service {
            uri: hyper::Uri::from_static("http://localhost:8081"),
        }
    }

    pub(crate) fn uri(&self) -> &hyper::Uri {
        &self.uri
    }
}
