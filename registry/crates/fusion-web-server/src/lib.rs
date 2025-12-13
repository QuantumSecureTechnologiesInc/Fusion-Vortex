use axum::Router as AxumRouter;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// Re-export specific axum types that we expose
pub use axum::extract;
pub use axum::response::Json;

/// Production-ready Web Server wrapper around Axum
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    /// Bind to a specific address
    pub fn bind(addr: &str) -> Self {
        let addr: SocketAddr = addr.parse().expect("Invalid address format");
        Self { addr }
    }

    /// Run the server with the given router
    pub async fn serve(self, app: Router) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.addr).await?;
        tracing::info!("Server listening on {}", self.addr);
        axum::serve(listener, app.0).await?;
        Ok(())
    }
}

/// Router wrapper to verify proper Fusion compilation flow
pub struct Router(AxumRouter);

impl Router {
    pub fn new() -> Self {
        Self(AxumRouter::new())
    }

    pub fn route<H, T>(self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        Self(self.0.route(path, handler))
    }

    pub fn into_make_service(self) -> AxumRouter {
        self.0
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export HTTP method helpers
pub fn post<H, T>(handler: H) -> axum::routing::MethodRouter
where
    H: axum::handler::Handler<T, ()>,
    T: 'static,
{
    axum::routing::post(handler)
}

pub fn get<H, T>(handler: H) -> axum::routing::MethodRouter
where
    H: axum::handler::Handler<T, ()>,
    T: 'static,
{
    axum::routing::get(handler)
}
