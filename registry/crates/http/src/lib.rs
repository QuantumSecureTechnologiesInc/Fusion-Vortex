use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder as ServerBuilder;
use std::convert::Infallible;
use std::future::Future;
/// Lightweight HTTP Server and Client wrapper around Hyper
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::TcpListener;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Simple HTTP Server
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self> {
        let socket_addr: SocketAddr = addr.parse()?;
        Ok(Self { addr: socket_addr })
    }

    pub async fn serve(&self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        println!("Listening on http://{}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                let service = RequestHandler;
                if let Err(err) = ServerBuilder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

#[derive(Clone)]
struct RequestHandler;

impl Service<Request<Incoming>> for RequestHandler {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        Box::pin(async move {
            let body = format!("Hello from Fusion! You requested: {}", req.uri());
            Ok(Response::new(Full::new(Bytes::from(body))))
        })
    }
}

/// Simple HTTP Client
pub struct Client {
    client: hyper_util::client::legacy::Client<
        hyper_util::client::legacy::connect::HttpConnector,
        Full<Bytes>,
    >,
}

impl Client {
    pub fn new() -> Self {
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();
        Self { client }
    }

    pub async fn get(&self, uri: &str) -> Result<String> {
        let uri = uri.parse()?;
        let resp = self.client.get(uri).await?;
        let body = resp.into_body().collect().await?.to_bytes();
        Ok(String::from_utf8(body.to_vec())?)
    }

    pub async fn post(&self, uri: &str, body: String) -> Result<String> {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .body(Full::new(Bytes::from(body)))?;

        let resp = self.client.request(req).await?;
        let body = resp.into_body().collect().await?.to_bytes();
        Ok(String::from_utf8(body.to_vec())?)
    }
}
