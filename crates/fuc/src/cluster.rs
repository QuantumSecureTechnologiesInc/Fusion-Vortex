use std::collections::HashSet;
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;
use parking_lot::Mutex;
use anyhow::Result;

pub struct ClusterNodeMetadata {
    pub unique_identity: String,
    pub network_address: String,
}

pub struct ClusterMembershipRegistry {
    known_identities: Arc<Mutex<HashSet<String>>>,
    broadcast_socket: UdpSocket,
}

impl ClusterMembershipRegistry {
    pub fn launch_registry(local_identity: &str, cluster_broadcast_addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        socket.set_read_timeout(Some(Duration::from_millis(500)))?;

        let known_identities = Arc::new(Mutex::new(HashSet::new()));
        let map_handle = Arc::clone(&known_identities);
        let id_string = local_identity.to_string();
        let target_addr = cluster_broadcast_addr.to_string();

        // Broadcast node presence heartbeat actively to network plane
        std::thread::spawn(move || {
            let heartbeat_payload = format!("ALIVE:{}", id_string);
            loop {
                let _ = socket.send_to(heartbeat_payload.as_bytes(), &target_addr);
                std::thread::sleep(Duration::from_secs(2));
            }
        });

        Ok(Self {
            known_identities: map_handle,
            broadcast_socket: UdpSocket::bind("0.0.0.0:8082")?,
        })
    }

    pub fn audit_active_cluster_members(&self) -> Vec<String> {
        let mut buffer = [0u8; 256];
        if let Ok((amt, _)) = self.broadcast_socket.recv_from(&mut buffer) {
            let trace = String::from_utf8_lossy(&buffer[..amt]);
            if trace.starts_with("ALIVE:") {
                let node_id = trace[6..].to_string();
                self.known_identities.lock().insert(node_id);
            }
        }
        self.known_identities.lock().iter().cloned().collect()
    }
}