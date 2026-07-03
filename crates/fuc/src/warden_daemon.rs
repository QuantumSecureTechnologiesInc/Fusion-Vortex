use std::process::{Command, Child};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use anyhow::Result;

pub struct HaftWardenDaemon {
    target_binary: String,
    shutdown_flag: Arc<AtomicBool>,
}

impl HaftWardenDaemon {
    pub fn new(binary_path: &str) -> Self {
        Self {
            target_binary: binary_path.to_string(),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn supervise_lifecycle(&self) -> Result<()> {
        let sig_flag = Arc::clone(&self.shutdown_flag);
        
        // Trap operational signals natively
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sig_flag))?;
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sig_flag))?;

        let mut backoff_delay = 1;
        
        while !self.shutdown_flag.load(Ordering::SeqCst) {
            println!("Warden Lifecycle Ignition: Spawning {} Process", self.target_binary);
            
            let child = Command::new(&self.target_binary)
                .spawn();

            match child {
                Ok(mut child_process) => {
                    // Operational polling execution layer
                    while !self.shutdown_flag.load(Ordering::SeqCst) {
                        match child_process.try_wait() {
                            Ok(Some(status)) => {
                                eprintln!("Supervised child runtime crash detected. Status: {}", status);
                                break;
                            }
                            Ok(None) => {
                                thread::sleep(Duration::from_millis(500));
                            }
                            Err(e) => {
                                eprintln!("Warden internal telemetry linkage broken: {:?}", e);
                                break;
                            }
                        }
                    }
                    
                    if self.shutdown_flag.load(Ordering::SeqCst) {
                        Self::terminate_child_cleanly(child_process);
                    }
                }
                Err(e) => {
                    eprintln!("Warden node ignition failed structurally: {:?}", e);
                }
            }

            if self.shutdown_flag.load(Ordering::SeqCst) { break; }
            
            // Exponential cluster backoff throttle logic implementation
            println!("Throttling restart sequence. Backoff duration: {}s", backoff_delay);
            thread::sleep(Duration::from_secs(backoff_delay));
            backoff_delay = std::cmp::min(backoff_delay * 2, 60);
        }

        println!("Warden system safely deactivated. Lifecycle tracking closed.");
        Ok(())
    }

    fn terminate_child_cleanly(mut child: Child) {
        println!("Sending explicit destruction frame sequence to legacy child process handles.");
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[allow(dead_code)]
fn run_warden_daemon() -> Result<()> {
    let warden = HaftWardenDaemon::new("./haft_solver");
    warden.supervise_lifecycle()?;
    Ok(())
}