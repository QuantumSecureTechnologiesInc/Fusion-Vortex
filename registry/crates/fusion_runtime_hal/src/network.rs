//! Network interface with ultra-low latency support

use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;
use tracing::{debug, trace};

/// Network interface supporting standard sockets and optional DPDK
pub struct NetworkInterface {
    enable_dpdk: bool,
}

impl NetworkInterface {
    pub fn new(enable_dpdk: bool) -> Self {
        if enable_dpdk {
            debug!("Network interface with DPDK enabled (requires root)");
        } else {
            debug!("Network interface using standard sockets");
        }

        Self { enable_dpdk }
    }

    /// Create a TCP socket
    pub fn tcp_socket(&self, addr: SocketAddr) -> Result<Socket, std::io::Error> {
        trace!("Creating TCP socket for {}", addr);

        let domain = if addr.is_ipv4() {
            Domain::IPV4
        } else {
            Domain::IPV6
        };

        let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;

        // Enable low-latency options
        socket.set_nodelay(true)?;

        Ok(socket)
    }

    /// Create a UDP socket
    pub fn udp_socket(&self, addr: SocketAddr) -> Result<Socket, std::io::Error> {
        trace!("Creating UDP socket for {}", addr);

        let domain = if addr.is_ipv4() {
            Domain::IPV4
        } else {
            Domain::IPV6
        };

        Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))
    }

    /// Check if DPDK is enabled
    pub fn is_dpdk_enabled(&self) -> bool {
        self.enable_dpdk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let network = NetworkInterface::new(false);
        assert!(!network.is_dpdk_enabled());
    }
}
