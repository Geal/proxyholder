use std::net::SocketAddr;

pub(crate) struct Configuration {
    pub(crate) listener: Listener,
}

impl Configuration {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Configuration {
            listener: Listener {
                address: SocketAddr::from(([127, 0, 0, 1], 8080)),
            },
        })
    }
}

pub(crate) struct Listener {
    pub(crate) address: SocketAddr,
}
