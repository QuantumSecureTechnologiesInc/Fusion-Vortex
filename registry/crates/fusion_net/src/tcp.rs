use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::OnceLock;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::runtime::Runtime;
use tracing::{debug, error, trace};

// Global Tokio Runtime for IO driving
fn get_io_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Fusion Net IO runtime")
    })
}

/// TCP stream using Tokio for cross-platform support (Linux/macOS/Windows)
pub struct TcpStream {
    inner: tokio::net::TcpStream,
}

impl TcpStream {
    /// Connect to a remote address
    pub async fn connect(addr: SocketAddr) -> io::Result<Self> {
        debug!("Connecting TCP stream to {}", addr);
        // Offload creation to the IO runtime to ensure driver registration
        let handle = get_io_runtime().handle();

        let stream = handle
            .spawn(async move { tokio::net::TcpStream::connect(addr).await })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))??;

        Ok(Self { inner: stream })
    }

    /// Read data from the stream
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use tokio::io::AsyncReadExt;
        self.inner.read(buf).await
    }

    /// Write data to the stream
    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use tokio::io::AsyncWriteExt;
        self.inner.write(buf).await
    }
}

// Implement AsyncRead and AsyncWrite for compatibility
impl AsyncRead for TcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// TCP listener using Tokio for cross-platform support
pub struct TcpListener {
    inner: tokio::net::TcpListener,
}

impl TcpListener {
    /// Bind to a local address
    pub async fn bind(addr: SocketAddr) -> io::Result<Self> {
        debug!("Binding TCP listener to {}", addr);
        let handle = get_io_runtime().handle();

        let listener = handle
            .spawn(async move { tokio::net::TcpListener::bind(addr).await })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))??;

        Ok(Self { inner: listener })
    }

    /// Accept an incoming connection
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        // We can call accept directly as the listener is already bound to the IO runtime
        let (stream, addr) = self.inner.accept().await?;
        trace!("Accepted connection from {}", addr);
        Ok((TcpStream { inner: stream }, addr))
    }
}
