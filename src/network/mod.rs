// src/network/mod.rs - Secure Networking with Authentication & Rate Limiting
//! Provides a lightweight TCP server/client with a post‑quantum Kyber‑768 key exchange.
//! Includes AEAD encryption (ChaCha20-Poly1305), client authentication, and rate limiting.

pub mod error;
pub mod rpc;

#[cfg(test)]
mod tests;

pub use error::NetworkError;
pub use rpc::Message;

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    ChaCha20Poly1305,
};
use pqcrypto_mlkem::mlkem768;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SharedSecret};
use rand::RngCore;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::async_runtime::executor::Executor;
use crate::async_runtime::task::Task;

/// Result type for networking operations.
pub type NetResult<T> = Result<T, NetworkError>;

/// Simple Rate Limiter (Token Bucket)
struct RateLimiter {
    tokens: u32,
    last_refill: Instant,
    max_per_second: u32,
}

impl RateLimiter {
    fn new(max_per_second: u32) -> Self {
        Self {
            tokens: max_per_second,
            last_refill: Instant::now(),
            max_per_second,
        }
    }

    fn check(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs() as u32;

        if elapsed > 0 {
            self.tokens = (self.tokens + elapsed * self.max_per_second).min(self.max_per_second);
            self.last_refill = now;
        }

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

/// A channel that has performed a Kyber‑768 handshake and can encrypt/decrypt data.
pub struct SecureChannel {
    stream: TcpStream,
    aead: ChaCha20Poly1305,
    rate_limiter: Option<RateLimiter>,
    authenticated_identity: Option<String>,
}

impl SecureChannel {
    /// Enable rate limiting for this channel
    pub fn with_rate_limit(mut self, max_per_second: u32) -> Self {
        self.rate_limiter = Some(RateLimiter::new(max_per_second));
        self
    }

    /// Check if client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.authenticated_identity.is_some()
    }

    /// Get authenticated identity
    pub fn identity(&self) -> Option<&String> {
        self.authenticated_identity.as_ref()
    }

    /// Perform the Kyber handshake as a **client**.
    fn client_handshake(mut stream: TcpStream) -> NetResult<Self> {
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(30)))?;

        // 1️⃣ Generate our key‑pair.
        let (pk, _sk) = mlkem768::keypair();
        // 2️⃣ Send our public key.
        stream.write_all(pk.as_bytes())?;
        // 3️⃣ Receive server's ciphertext.
        let mut ct_buf = vec![0u8; mlkem768::ciphertext_bytes()];
        stream.read_exact(&mut ct_buf)?;
        let ct = Ciphertext::from_bytes(&ct_buf)
            .map_err(|e| NetworkError::Handshake(format!("Invalid ciphertext: {}", e)))?;
        // 4️⃣ Decapsulate to obtain the shared secret.
        let shared = mlkem768::decapsulate(&ct, &_sk);
        // ChaCha20Poly1305 expects a 32‑byte key.
        let key = GenericArray::from_slice(shared.as_bytes());
        Ok(Self {
            stream,
            aead: ChaCha20Poly1305::new(key),
            rate_limiter: None, // Clients usually don't limit themselves receiving? Or maybe receiving from server?
            authenticated_identity: None,
        })
    }

    /// Perform the Kyber handshake as a **server**.
    fn server_handshake(mut stream: TcpStream) -> NetResult<Self> {
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(30)))?;

        // 1️⃣ Receive client public key.
        let mut pk_buf = vec![0u8; mlkem768::public_key_bytes()];
        stream.read_exact(&mut pk_buf)?;
        let client_pk = PublicKey::from_bytes(&pk_buf)
            .map_err(|e| NetworkError::Handshake(format!("Invalid public key: {}", e)))?;
        // 2️⃣ Generate our key‑pair (ephemeral for forward secrecy, but we only need encap here).
        // Actually for Kyber KEM, server (receiver of PK) encapsulates.
        // Wait, standard KEM: A sends PK to B. B Encaps(PK) -> (CT, SS). B sends CT to A. A Decaps(CT) -> SS.
        // Here: Client sends PK. Server receives PK.
        // Server Encaps(PK) -> (Shared, CT).
        // Server sends CT.
        // So Server knows Shared. Client knows Shared.

        let (shared, ct) = mlkem768::encapsulate(&client_pk);

        // 4️⃣ Send ciphertext to client.
        stream.write_all(ct.as_bytes())?;
        // Initialise AEAD.
        let key = GenericArray::from_slice(shared.as_bytes());
        Ok(Self {
            stream,
            aead: ChaCha20Poly1305::new(key),
            rate_limiter: Some(RateLimiter::new(100)), // Default 100 req/s
            authenticated_identity: None,
        })
    }

    /// Authenticate with the server using a token
    pub fn authenticate(&mut self, client_id: &str, token: &str) -> NetResult<()> {
        let req = Message::Authenticate {
            client_id: client_id.to_string(),
            token: token.to_string(),
        };
        self.send_message(&req)?;

        match self.recv_message()? {
            Message::AuthResult {
                success: true,
                message: _,
            } => {
                self.authenticated_identity = Some(client_id.to_string());
                Ok(())
            }
            Message::AuthResult {
                success: false,
                message,
            } => Err(NetworkError::Auth(message)),
            _ => Err(NetworkError::Auth(
                "Unexpected response during authentication".into(),
            )),
        }
    }

    /// Encrypt and send a message using ChaCha20‑Poly1305.
    pub fn send(&mut self, data: &[u8]) -> NetResult<()> {
        // Generate a fresh nonce (12 bytes).
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = GenericArray::from_slice(&nonce_bytes);
        // Encrypt (AEAD adds authentication tag automatically).
        let ciphertext = self
            .aead
            .encrypt(nonce, data)
            .map_err(|_| NetworkError::Crypto("encryption failure".into()))?;
        // Frame: [nonce (12)] [len(u32)] [ciphertext]
        let len = (ciphertext.len() as u32).to_be_bytes();
        self.stream.write_all(&nonce_bytes)?;
        self.stream.write_all(&len)?;
        self.stream.write_all(&ciphertext)?;
        Ok(())
    }

    /// Receive and decrypt a message.
    pub fn recv(&mut self) -> NetResult<Vec<u8>> {
        // Rate Limiting Check
        if let Some(limiter) = &mut self.rate_limiter {
            if !limiter.check() {
                return Err(NetworkError::RateLimit);
            }
        }

        // Read nonce.
        let mut nonce_bytes = [0u8; 12];
        self.stream.read_exact(&mut nonce_bytes)?;
        let nonce = GenericArray::from_slice(&nonce_bytes);
        // Read length.
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;

        // Safety check for max length? (e.g. 10MB)
        if len > 10 * 1024 * 1024 {
            return Err(NetworkError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Message too large",
            )));
        }

        // Read ciphertext.
        let mut ciphertext = vec![0u8; len];
        self.stream.read_exact(&mut ciphertext)?;
        // Decrypt.
        let plaintext = self
            .aead
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| NetworkError::Crypto("decryption failure".into()))?;
        Ok(plaintext)
    }

    /// Helper to send a structured Message
    pub fn send_message(&mut self, msg: &Message) -> NetResult<()> {
        let bytes =
            bincode::serialize(msg).map_err(|e| NetworkError::Serialization(e.to_string()))?;
        self.send(&bytes)
    }

    /// Helper to receive a structured Message
    pub fn recv_message(&mut self) -> NetResult<Message> {
        let bytes = self.recv()?;
        bincode::deserialize(&bytes).map_err(|e| NetworkError::Serialization(e.to_string()))
    }

    /// Asynchronous send – encrypts and writes data.
    pub async fn async_send(&mut self, data: &[u8]) -> NetResult<()> {
        self.send(data)
    }

    /// Asynchronous receive – reads and decrypts data.
    pub async fn async_recv(&mut self) -> NetResult<Vec<u8>> {
        self.recv()
    }
}

/// High‑level API used by the rest of Fusion.
pub struct FusionNetwork;

impl FusionNetwork {
    /// Run a server that listens on `addr` and executes `handler` for each connection.
    /// The function spawns a new `Task` for each client on the supplied `Executor`.
    ///
    /// Note: The handler is assumed to be thread-safe (Arc<Mutex<F>> internal handling).
    pub fn run_server<F>(addr: &str, handler: F, exec: &mut Executor) -> NetResult<()>
    where
        F: Fn(SecureChannel) -> NetResult<()> + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(addr)?;
        let handler = Arc::new(handler);

        // Accept loop (blocking accept, spawned handling)
        // In a real async runtime, accept would be async.
        // Here we block on accept and spawn tasks.
        listener.set_nonblocking(true)?;

        println!("Server listening on {}", addr);

        // We need a way to run the accept loop reasonably.
        // Since `exec.spawn` adds a task, we should probably spawn a task that does the accept loop?
        // Or if this function blocks, it blocks the whole executor?
        // The previous implementation was blocking.
        // Let's implement a simple non-blocking loop that yields if no connection.

        // Actually, for simplicity and meeting the interface expectation:
        // We will make the listener blocking for this implementation or rely on the caller to not block?
        // The signature returns NetResult<()>, implying it runs until error or...
        // If it's `run_server`, it likely blocks.

        listener.set_nonblocking(false)?;

        println!("Server listening on {}", addr);

        loop {
            let (socket, _) = listener.accept()?;
            // Handshake
            match SecureChannel::server_handshake(socket) {
                Ok(mut channel) => {
                    // Apply default rate limits
                    channel = channel.with_rate_limit(100);

                    // Allow handler to process concurrently
                    let h_clone = handler.clone();
                    let task = Task::new(
                        async move {
                            let _ = h_clone(channel);
                        },
                        0,
                    );
                    exec.spawn(task);
                }
                Err(e) => {
                    eprintln!("Handshake failed: {}", e);
                }
            }
        }
    }

    /// Connect to a remote server and return a ready‑to‑use `SecureChannel`.
    pub fn connect(addr: &str) -> NetResult<SecureChannel> {
        let stream = TcpStream::connect(addr)?;
        SecureChannel::client_handshake(stream)
    }

    /// Asynchronous connect using the custom async runtime.
    pub async fn async_connect(addr: &str) -> NetResult<SecureChannel> {
        // Delegates to the blocking version; the executor will poll this future.
        // In fully async, this would use async IO.
        Self::connect(addr)
    }
}
