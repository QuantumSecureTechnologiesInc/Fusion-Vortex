use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
pub use hyper::body::Incoming;
use hyper::service::Service;
pub use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder as ServerBuilder;
use std::convert::Infallible;
use std::future::Future;
/// Lightweight HTTP Server and Client wrapper around Hyper
use std::pin::Pin;
use tokio::net::TcpListener;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Simple HTTP Server
pub struct Server<H> {
    handler: H,
}

impl<H> Server<H>
where
    H: Fn(Request<Vec<u8>>) -> Response<Vec<u8>> + Clone + Send + Sync + 'static,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub async fn listen(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);

        let handler = self.handler.clone();

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let handler_clone = handler.clone();

            tokio::task::spawn(async move {
                let service = RequestHandler {
                    handler: handler_clone,
                };
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
struct RequestHandler<H> {
    handler: H,
}

impl<H> Service<Request<Incoming>> for RequestHandler<H>
where
    H: Fn(Request<Vec<u8>>) -> Response<Vec<u8>> + Clone + Send + Sync + 'static,
{
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move {
            // Collect body
            let (parts, body) = req.into_parts();
            let bytes = match body.collect().await {
                Ok(c) => c.to_bytes().to_vec(),
                Err(_) => Vec::new(), // Should handle error properly
            };
            let req_vec = Request::from_parts(parts, bytes);

            // Call handler
            let resp = handler(req_vec);

            // Convert Response<Vec<u8>> to Response<Full<Bytes>>
            let (parts, body) = resp.into_parts();
            let body_full = Full::new(Bytes::from(body));
            Ok(Response::from_parts(parts, body_full))
        })
    }
}

/// Simple HTTP Client
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, uri: &str) -> Result<String> {
        let resp = self.client.get(uri).send().await?;
        Ok(resp.text().await?)
    }

    pub async fn post(&self, uri: &str, body: String) -> Result<String> {
        let resp = self.client.post(uri).body(body).send().await?;
        Ok(resp.text().await?)
    }
}
