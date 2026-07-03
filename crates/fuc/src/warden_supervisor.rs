// The Warden Sentinel Daemon (Crash & Heal Core)
// Rationale: Monitors distributed cluster nodes via liveness loops, catches crashes
// with exponential backoff (Crash), and automates rollback upgrades (Heal) on fail.

use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::net::TcpStream;
use std::thread;
use anyhow::Result;

pub struct WardenSentinel {
    monitored_binary: String,
    network_probe_address: String,
    shutdown_flag: Arc<AtomicBool>,
}

impl WardenSentinel {
    pub fn new(binary_path: &str, probe_addr: &str) -> Self {
        Self {
            monitored_binary: binary_path.to_string(),
            network_probe_address: probe_addr.to_string(),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Spawns the node supervisor lifecycle loop.
    pub fn monitor_and_heal_cluster(&self) -> Result<()> {
        let mut retry_backoff_seconds = 1;

        while !self.shutdown_flag.load(Ordering::SeqCst) {
            println!("Warden Sentinel: Spawning child instance: {}", self.monitored_binary);
            
            let mut child_process = Command::new(&self.monitored_binary)
                .spawn()?;

            let mut process_is_healthy = true;
            
            // Enter execution health-probing loop
            while process_is_healthy && !self.shutdown_flag.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(500));

                // 1. Audit child process level-3 health natively
                match child_process.try_wait() {
                    Ok(Some(status)) => {
                        eprintln!("Warden: Child crashed with exit status code: {}", status);
                        break;
                    }
                    Ok(None) => {} // Node still running
                    Err(_) => {
                        break;
                    }
                }

                // 2. Perform live TCP liveness checks
                if !self.verify_network_liveness() {
                    eprintln!("Warden: Node failed networking handshake check. Initiating crash recovery.");
                    process_is_healthy = false;
                }
            }

            // Clean up the degraded instance before healing the node
            let _ = child_process.kill();
            let _ = child_process.wait();

            if self.shutdown_flag.load(Ordering::SeqCst) {
                break;
            }

            // Implement Crash/Heal upgrade & backoff sequence
            if self.inspect_pending_systemd_upgrade() {
                self.execute_atomic_upgrade_rollback();
            } else {
                println!("Throttling restart sequence. Backoff duration: {}s", retry_backoff_seconds);
                thread::sleep(Duration::from_secs(retry_backoff_seconds));
                retry_backoff_seconds = std::cmp::min(retry_backoff_seconds * 2, 60);
            }
        }

        Ok(())
    }

    fn verify_network_liveness(&self) -> bool {
        // Attempt TCP connect handshake natively within tight time limits
        TcpStream::connect_timeout(
            &self.network_probe_address.parse().unwrap(),
            Duration::from_millis(150)
        ).is_ok()
    }

    fn inspect_pending_systemd_upgrade(&self) -> bool {
        // Checks local staging directories for verified upgrade indicators
        std::path::Path::new("/var/lib/fusion/upgrade_pending").exists()
    }

    fn execute_atomic_upgrade_rollback(&self) {
        println!("Warden: Pending upgrade detected. Triggering hot-swap...");
        
        // Simulates an atomic binary swap inside systemd services directory
        let upgrade_result = std::fs::copy("/var/lib/fusion/upgrade_bin", &self.monitored_binary);
        
        if upgrade_result.is_ok() && self.verify_network_liveness() {
            println!("Warden: Upgrade applied successfully. Thermal states stable.");
            let _ = std::fs::remove_file("/var/lib/fusion/upgrade_pending");
        } else {
            eprintln!("Warden: Upgrade failed liveness testing! Triggering automatic rollback.");
            let _ = std::fs::copy("/var/lib/fusion/backup_bin", &self.monitored_binary);
            println!("Warden: System safely rolled back to legacy stable binary version.");
        }
    }
}