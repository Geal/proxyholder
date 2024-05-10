use std::convert::Infallible;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;

use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::client::conn::http1::Builder;
use hyper::rt::Executor;
use hyper::server::conn::http1;
use hyper::service::{service_fn, Service};
use hyper::upgrade::Upgraded;
use hyper::{Method, Request, Response, StatusCode, Uri};
use hyper_util::rt::tokio::TokioIo;
use hyper_util::rt::TokioExecutor;
use hyper_util::server::conn::auto;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

use crate::router::Router;

pub(crate) struct Session<Stream: AsyncRead + AsyncWrite + Unpin> {
    stream: TokioIo<Stream>,
}

impl<Stream: AsyncRead + AsyncWrite + Unpin> Session<Stream> {
    pub(crate) fn new(stream: TokioIo<Stream>) -> Self {
        Self { stream }
    }

    pub(crate) async fn run(&mut self, router: Arc<Router>) {
        let stream = pin!(&mut self.stream);
        // Finally, we bind the incoming connection to our `hello` service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            //.serve_connection(stream, service_fn(hello))
            .serve_connection(stream, service_fn(proxy))
            .await
        {
            tracing::error!(error = %err, "Error serving connection",);
        }
        tracing::info!("served connection");
    }
}

type RequestIn = Request<Incoming>;
type ResponseOut = Response<Full<Bytes>>;

pub(crate) struct SessionService {
    router: Arc<Router>,
}

impl Service<RequestIn> for SessionService {
    type Response = ResponseOut;
    type Error = Infallible;
    type Future = Box<Pin<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    fn call(&self, req: RequestIn) -> Self::Future {
        println!("processing request: {} {}", req.method(), req.uri().path());
        //self.inner.call(req)
        Box::pin(hello(req))
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<ResponseOut, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
