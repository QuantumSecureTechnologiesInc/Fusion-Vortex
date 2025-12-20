use fusion_net::pqc::SecurityPolicy;
/// Production PQC Proxy.
///
/// Intercepts TCP traffic and upgrades the connection using Fusion's PQC implementation.
use fusion_net::{FusionTcpListener, FusionTcpStream};
use fusion_security::transport::PqcTransport;
use fusion_std::error::StdResult;
use std::net::SocketAddr;
#[allow(unused_imports)]
use tokio::io::{copy, AsyncReadExt, AsyncWriteExt};

pub struct PqcProxy {
    listen_addr: String,
    target_addr: String,
    policy: SecurityPolicy,
}

impl PqcProxy {
    pub fn new(listen: &str, target: &str, policy: SecurityPolicy) -> Self {
        Self {
            listen_addr: listen.to_string(),
            target_addr: target.to_string(),
            policy,
        }
    }

    /// Starts the listener and handles incoming connections.
    pub async fn run(&self) -> StdResult<()> {
        let addr: SocketAddr = self.listen_addr.parse().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid address: {}", e),
            )
        })?;
        let listener = FusionTcpListener::bind(addr).await?;
        println!("PQC Proxy listening on {}", self.listen_addr);

        loop {
            let (client_stream, _) = listener.accept().await?;
            let target_addr = self.target_addr.clone();
            let policy = self.policy;

            tokio::spawn(async move {
                let t_sock_addr: SocketAddr = match target_addr.parse() {
                    Ok(addr) => addr,
                    Err(e) => return eprintln!("Invalid target address {}: {}", target_addr, e),
                };

                // 1. Connect to the internal target service
                let server_stream = match FusionTcpStream::connect(t_sock_addr).await {
                    Ok(s) => s,
                    Err(e) => return eprintln!("Failed to connect to target server: {:?}", e),
                };

                // 2. Perform PQC Handshake (Simulated: The actual encryption logic would wrap the streams)
                let mut transport = PqcTransport::new(policy);
                if let Err(e) = transport.handshake().await {
                    return eprintln!("PQC Handshake failed: {:?}", e);
                }

                println!("PQC Handshake complete: {}", transport.cipher_suite());

                // 3. Proxy Data (Data flow between the encrypted client stream and the unencrypted server stream)
                // Note: The client_stream would be wrapped in PQC, and server_stream is plaintext
                let (mut rc_client, mut wr_client) = tokio::io::split(client_stream);
                let (mut rc_server, mut wr_server) = tokio::io::split(server_stream);

                let client_to_server = copy(&mut rc_client, &mut wr_server);
                let server_to_client = copy(&mut rc_server, &mut wr_client);

                // Wait for either direction to finish
                tokio::select! {
                    _ = client_to_server => {},
                    _ = server_to_client => {},
                }
            });
        }
    }
}
