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

pub(crate) struct Proxy {}

impl Proxy {
    pub(crate) async fn new(
        configuration: Arc<Configuration>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        //FIXME: check NO_DELAY
        let listener = TcpListener::bind(configuration.listener.address).await?;

        println!(
            "starting loop listening on {}",
            configuration.listener.address
        );
        loop {
            let (stream, _) = listener.accept().await?;
            println!("got listener");

            let io = TokioIo::new(stream);

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, service_fn(hello))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
                println!("served connection");
            });
        }
        Ok(Proxy {})
    }

    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
