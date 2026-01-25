// Status: Production Supervisor (Sentinel)
// Purpose: The "Immune System" - Monitors child processes with Backoff, Signals, and Pre-flight Checks.
// Real Implementation: Uses libc for signals, std::process for supervision, and active TCP probing.

use std::process::{Command, Child};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use fusion_runtime_core::Runtime;
use anyhow::{anyhow, Result};

// --- Signal Handling (using libc from dependencies) ---
static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_signal(_: i32) {
    // Atomic flag set for safe shutdown in the main loop
    SHUTDOWN_SIGNAL.store(true, Ordering::SeqCst);
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(libc::SIGINT, handle_signal as usize);
        libc::signal(libc::SIGTERM, handle_signal as usize);
    }
}

// --- Logging Helper ---
fn log(level: &str, msg: &str) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("[{}] [{}] {}", now, level, msg);
}

// --- Health Strategy ---
#[derive(Clone, Debug)]
enum HealthCheckStrategy {
    ProcessOnly,
    TcpProbe(u16),
}

// --- Service Definition ---
#[derive(Clone, Debug)]
struct ServiceConfig {
    name: String,
    binary_path: String,
    args: Vec<String>,
    health_check: HealthCheckStrategy,
}

// --- Runtime State for a Service ---
struct ServiceState {
    child: Option<Child>,
    restart_count: u32,
    last_restart: Instant,
}

impl ServiceState {
    fn new() -> Self {
        Self {
            child: None,
            restart_count: 0,
            last_restart: Instant::now(),
        }
    }
}

// --- The Warden Supervisor ---
struct Warden {
    configs: Vec<ServiceConfig>,
    /// Map of Service Name -> Runtime State
    state: Mutex<HashMap<String, ServiceState>>,
}

impl Warden {
    fn new() -> Self {
        // Production: Load paths from Env or default to relative
        // Validates that we can override these for testing or container paths
        let nexus_bin = env::var("FUSION_NEXUS_BIN").unwrap_or_else(|_| "./haft_nexus".to_string());
        let vault_bin = env::var("FUSION_VAULT_BIN").unwrap_or_else(|_| "./haft_vault".to_string());
        let solver_bin = env::var("FUSION_SOLVER_BIN").unwrap_or_else(|_| "./haft_solver".to_string());

        let configs = vec![
            ServiceConfig {
                name: "Nexus".to_string(),
                binary_path: nexus_bin,
                args: vec![],
                health_check: HealthCheckStrategy::TcpProbe(8080),
            },
            ServiceConfig {
                name: "Vault".to_string(),
                binary_path: vault_bin,
                args: vec![],
                health_check: HealthCheckStrategy::TcpProbe(8081),
            },
            ServiceConfig {
                name: "Solver-1".to_string(),
                binary_path: solver_bin,
                args: vec!["--id".to_string(), "1".to_string()],
                health_check: HealthCheckStrategy::ProcessOnly,
            },
        ];

        Self {
            configs,
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Pre-flight check to ensure environment is valid.
    fn verify_binaries(&self) -> Result<()> {
        log("INFO", "Running pre-flight binary verification...");
        for cfg in &self.configs {
            let path = Path::new(&cfg.binary_path);
            if !path.exists() {
                let msg = format!("Missing required binary for {}: {:?}", cfg.name, path);
                log("FATAL", &msg);
                return Err(anyhow!(msg));
            }
        }
        log("INFO", "All binaries verified.");
        Ok(())
    }

    /// Bootstraps the cluster.
    fn boot(&self) -> Result<()> {
        self.verify_binaries()?;
        register_signal_handlers();
        
        let mut state_map = self.state.lock().unwrap();
        
        for cfg in &self.configs {
            log("BOOT", &format!("Starting service: {} (Path: {})", cfg.name, cfg.binary_path));
            let child = self.spawn_process(cfg)?;
            
            let mut state = ServiceState::new();
            state.child = Some(child);
            state_map.insert(cfg.name.clone(), state);
        }
        Ok(())
    }

    fn spawn_process(&self, cfg: &ServiceConfig) -> std::io::Result<Child> {
        Command::new(&cfg.binary_path)
            .args(&cfg.args)
            .spawn()
    }

    /// Main Monitoring Loop
    async fn monitor_loop(&self) {
        log("INFO", &format!("Supervisor Active. PID: {}", std::process::id()));
        
        while !SHUTDOWN_SIGNAL.load(Ordering::SeqCst) {
            self.check_services();
            thread::sleep(Duration::from_secs(1));
        }

        self.shutdown();
    }

    fn check_services(&self) {
        let mut state_map = self.state.lock().unwrap();

        for cfg in &self.configs {
            let state = state_map.get_mut(&cfg.name).unwrap();
            let mut needs_restart = false;
            let mut reason = "";

            if let Some(child) = &mut state.child {
                // Check 1: Process Exit Code (Fast check)
                match child.try_wait() {
                    Ok(Some(status)) => {
                        reason = "Exited";
                        log("ALERT", &format!("{} exited with {}", cfg.name, status));
                        needs_restart = true;
                    }
                    Ok(None) => {
                        // Check 2: Liveness Probe (Slow check)
                        if !self.perform_active_probe(cfg) {
                            reason = "Probe Failed";
                            log("ALERT", &format!("{} failed health probe. Killing...", cfg.name));
                            let _ = child.kill(); // Force kill zombie
                            let _ = child.wait(); // Reap zombie
                            needs_restart = true;
                        }
                    }
                    Err(e) => {
                        log("ERROR", &format!("Error waiting on {}: {}", cfg.name, e));
                        needs_restart = true;
                    }
                }
            } else {
                needs_restart = true;
            }

            if needs_restart {
                // Check Backoff
                if self.should_backoff(state) {
                    // Log backoff only once per cycle to avoid spamming
                    // (Real impl might use a flag, here we rely on the sleep interval)
                } else {
                    log("RESTART", &format!("Reviving {} (Reason: {})...", cfg.name, reason));
                    match self.spawn_process(cfg) {
                        Ok(new_child) => {
                            state.child = Some(new_child);
                            state.restart_count += 1;
                            state.last_restart = Instant::now();
                        }
                        Err(e) => log("CRITICAL", &format!("Failed to spawn {}: {}", cfg.name, e)),
                    }
                }
            } else {
                // Reset restart count if stable for > 60 seconds
                if state.last_restart.elapsed().as_secs() > 60 {
                    state.restart_count = 0;
                }
            }
        }
    }

    /// Simple exponential backoff guard.
    /// Returns true if we should wait before restarting.
    fn should_backoff(&self, state: &ServiceState) -> bool {
        let uptime = state.last_restart.elapsed().as_secs();
        let threshold = match state.restart_count {
            0..=2 => 0,   // Immediate restart for first 3 crashes
            3..=5 => 5,   // Wait 5s
            _ => 30,      // Wait 30s (Penalty box)
        };
        uptime < threshold
    }

    fn perform_active_probe(&self, cfg: &ServiceConfig) -> bool {
        match cfg.health_check {
            HealthCheckStrategy::ProcessOnly => true,
            HealthCheckStrategy::TcpProbe(port) => {
                let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
                TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok()
            }
        }
    }

    /// Graceful Shutdown Sequence
    fn shutdown(&self) {
        log("WARN", "Shutdown Signal Received. Terminating children...");
        let mut state_map = self.state.lock().unwrap();
        
        for (name, state) in state_map.iter_mut() {
            if let Some(child) = &mut state.child {
                log("INFO", &format!("Stopping {}...", name));
                // Try SIGTERM equivalent (kill)
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        log("INFO", "Shutdown Complete. Goodbye.");
    }
}

fn main() -> Result<()> {
    let runtime = Runtime::new();
    let warden = Arc::new(Warden::new());

    log("INFO", "Starting HAFT Warden (Production Supervisor)...");
    
    // Will panic/exit if binaries are missing (Safe Fail)
    if let Err(e) = warden.boot() {
        log("FATAL", &format!("Boot failed: {}", e));
        std::process::exit(1);
    }

    runtime.block_on(async move {
        warden.monitor_loop().await;
    });
    Ok(())
}

// --- Unit Tests (Compile with `cargo test --bin haft_warden`) ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_logic() {
        // Mock state
        let mut state = ServiceState::new();
        state.restart_count = 1;
        state.last_restart = Instant::now();

        let warden = Warden::new(); // Configs don't matter for this test

        // Case 1: Low restart count, immediate retry
        assert_eq!(warden.should_backoff(&state), false);

        // Case 2: High restart count, should wait
        state.restart_count = 10;
        state.last_restart = Instant::now(); // Just crashed
        assert_eq!(warden.should_backoff(&state), true);
    }

    #[test]
    fn test_env_override() {
        env::set_var("FUSION_NEXUS_BIN", "/tmp/mock_nexus");
        let warden = Warden::new();
        let nexus_cfg = warden.configs.iter().find(|c| c.name == "Nexus").unwrap();
        assert_eq!(nexus_cfg.binary_path, "/tmp/mock_nexus");
    }
}
