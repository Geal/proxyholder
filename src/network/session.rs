use std::convert::Infallible;
use std::pin::pin;
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
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

pub(crate) struct Session<Stream: AsyncRead + AsyncWrite + Unpin> {
    stream: TokioIo<Stream>,
}

impl<Stream: AsyncRead + AsyncWrite + Unpin> Session<Stream> {
    pub(crate) fn new(stream: TokioIo<Stream>) -> Self {
        Self { stream }
    }

    pub(crate) async fn run(&mut self) {
        let stream = pin!(&mut self.stream);
        // Finally, we bind the incoming connection to our `hello` service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            .serve_connection(stream, service_fn(hello))
            .await
        {
            tracing::error!(error = %err, "Error serving connection",);
        }
        tracing::info!("served connection");
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
