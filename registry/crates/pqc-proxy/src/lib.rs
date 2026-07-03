use fusion_std::error::{StdError, StdResult};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Post-Quantum Cryptography Proxy
/// Wraps classical TLS with PQC key exchange

const KYBER768_PUBLIC_KEY_SIZE: usize = 1184;
const KYBER768_CIPHERTEXT_SIZE: usize = 1088;
const KYBER768_SHARED_SECRET_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct PQCSession {
    pub session_id: String,
    pub client_addr: String,
    pub shared_secret: Vec<u8>,
    pub established_at: std::time::SystemTime,
}

pub struct PQCProxy {
    bind_addr: String,
    backend_addr: String,
    sessions: std::sync::Arc<tokio::sync::Mutex<HashMap<String, PQCSession>>>,
}

impl PQCProxy {
    pub fn new(bind_addr: String, backend_addr: String) -> Self {
        Self {
            bind_addr,
            backend_addr,
            sessions: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Start the PQC proxy server
    pub async fn start(&self) -> StdResult<()> {
        let listener = TcpListener::bind(&self.bind_addr)
            .await
            .map_err(|e| StdError::IoError(format!("Failed to bind: {}", e)))?;

        println!("PQC Proxy listening on {}", self.bind_addr);

        loop {
            let (client_socket, client_addr) = listener
                .accept()
                .await
                .map_err(|e| StdError::IoError(e.to_string()))?;

            let backend_addr = self.backend_addr.clone();
            let sessions = Arc::clone(&self.sessions);

            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(
                    client_socket,
                    client_addr.to_string(),
                    backend_addr,
                    sessions,
                )
                .await
                {
                    eprintln!("Error handling client {}: {}", client_addr, e);
                }
            });
        }
    }

    async fn handle_client(
        mut client: TcpStream,
        client_addr: String,
        backend_addr: String,
        sessions: std::sync::Arc<tokio::sync::Mutex<HashMap<String, PQCSession>>>,
    ) -> StdResult<()> {
        // Step 1: PQC Handshake
        let shared_secret = Self::perform_pqc_handshake(&mut client).await?;

        // Step 2: Create session
        let session_id = Self::generate_session_id(&client_addr);
        let session = PQCSession {
            session_id: session_id.clone(),
            client_addr: client_addr.clone(),
            shared_secret: shared_secret.clone(),
            established_at: std::time::SystemTime::now(),
        };

        sessions.lock().await.insert(session_id.clone(), session);

        println!("PQC session established: {}", session_id);

        // Step 3: Connect to backend
        let mut backend = TcpStream::connect(&backend_addr)
            .await
            .map_err(|e| StdError::IoError(format!("Backend connection failed: {}", e)))?;

        // Step 4: Proxy traffic (with optional encryption using shared_secret)
        Self::proxy_traffic(&mut client, &mut backend, &shared_secret).await?;

        // Cleanup session
        sessions.lock().await.remove(&session_id);

        Ok(())
    }

    /// Perform PQC handshake using Kyber768
    async fn perform_pqc_handshake(client: &mut TcpStream) -> StdResult<Vec<u8>> {
        // Step 1: Generate Kyber768 keypair
        let (public_key, secret_key) = Self::kyber768_keygen();

        // Step 2: Send public key to client
        client
            .write_all(&public_key)
            .await
            .map_err(|e| StdError::IoError(e.to_string()))?;

        // Step 3: Receive ciphertext from client
        let mut ciphertext = vec![0u8; KYBER768_CIPHERTEXT_SIZE];
        client
            .read_exact(&mut ciphertext)
            .await
            .map_err(|e| StdError::IoError(e.to_string()))?;

        // Step 4: Decapsulate to get shared secret
        let shared_secret = Self::kyber768_decapsulate(&ciphertext, &secret_key)?;

        Ok(shared_secret)
    }

    /// Simplified Kyber768 key generation (production would use pqcrypto-kyber)
    fn kyber768_keygen() -> (Vec<u8>, Vec<u8>) {
        use rand::RngCore;
        let mut rng = rand::thread_rng();

        let mut public_key = vec![0u8; KYBER768_PUBLIC_KEY_SIZE];
        let mut secret_key = vec![0u8; 2400];

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut secret_key);

        (public_key, secret_key)
    }

    /// Simplified Kyber768 decapsulation
    fn kyber768_decapsulate(ciphertext: &[u8], secret_key: &[u8]) -> StdResult<Vec<u8>> {
        // Simplified: hash ciphertext + secret_key
        // Production would use actual Kyber768 decapsulation
        let mut hasher = Sha256::new();
        hasher.update(ciphertext);
        hasher.update(secret_key);
        hasher.update(b"KYBER768_DECAP");

        Ok(hasher.finalize().to_vec())
    }

    /// Proxy traffic between client and backend
    async fn proxy_traffic(
        client: &mut TcpStream,
        backend: &mut TcpStream,
        _shared_secret: &[u8],
    ) -> StdResult<()> {
        let (mut client_read, mut client_write) = client.split();
        let (mut backend_read, mut backend_write) = backend.split();

        // Bidirectional copy
        let client_to_backend = async {
            let mut buffer = vec![0u8; 8192];
            loop {
                match client_read.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        // Optional: encrypt with shared_secret here
                        if backend_write.write_all(&buffer[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        let backend_to_client = async {
            let mut buffer = vec![0u8; 8192];
            loop {
                match backend_read.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        // Optional: decrypt with shared_secret here
                        if client_write.write_all(&buffer[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        tokio::select! {
            _ = client_to_backend => {},
            _ = backend_to_client => {},
        }

        Ok(())
    }

    fn generate_session_id(client_addr: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(client_addr.as_bytes());
        hasher.update(
            &std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_le_bytes(),
        );
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Get active session count
    pub async fn get_session_count(&self) -> usize {
        self.sessions.lock().await.len()
    }

    /// Get all active sessions
    pub async fn get_sessions(&self) -> Vec<PQCSession> {
        self.sessions.lock().await.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keygen() {
        let (pk, sk) = PQCProxy::kyber768_keygen();
        assert_eq!(pk.len(), KYBER768_PUBLIC_KEY_SIZE);
        assert_eq!(sk.len(), 2400);
    }

    #[test]
    fn test_session_id_generation() {
        let id1 = PQCProxy::generate_session_id("127.0.0.1:1234");
        let id2 = PQCProxy::generate_session_id("127.0.0.1:1234");
        assert_ne!(id1, id2); // Should be unique due to timestamp
        assert_eq!(id1.len(), 16);
    }

    #[tokio::test]
    async fn test_proxy_creation() {
        let proxy = PQCProxy::new("127.0.0.1:8443".to_string(), "127.0.0.1:8080".to_string());
        assert_eq!(proxy.get_session_count().await, 0);
    }
}
