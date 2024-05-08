use std::convert::Infallible;
use std::sync::Arc;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::Builder;
use hyper::rt::Executor;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::tokio::TokioIo;
use hyper_util::rt::TokioExecutor;
use hyper_util::server::conn::auto;
use tokio::net::TcpListener;

use crate::configuration::Configuration;
use crate::network::session::Session;
mod session;

pub(crate) struct Proxy {}

impl Proxy {
    pub(crate) async fn new(
        configuration: Arc<Configuration>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        //FIXME: check NO_DELAY
        let listener = TcpListener::bind(configuration.listener.address).await?;

        tracing::info!(
            address = %configuration.listener.address,
            "starting loop listening",
        );
        loop {
            let (stream, _) = listener.accept().await?;
            tracing::info!("got TCP stream");

            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                let mut session = Session::new(io);
                session.run().await;
            });
        }
        Ok(Proxy {})
    }

    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
