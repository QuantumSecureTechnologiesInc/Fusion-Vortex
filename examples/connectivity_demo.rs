// examples/connectivity_demo.rs
//! Demonstrates Fusion's custom async runtime networking with post‑quantum security.
//!
//! The demo starts a server in a background task, then a client connects, performs a
//! Kyber‑768 handshake, sends a message, and receives a reply.
//!
//! Run with:
//! ```bash
//! cargo run --example connectivity_demo
//! ```

use fusion_lang::async_runtime::executor::Executor;
use fusion_lang::network::FusionNetwork;
use std::thread;
use std::time::Duration;

use anyhow::Result;

fn main() -> Result<()> {
    // ------------------------------------------------------------
    // 1️⃣  Initialise a simple single‑threaded executor.
    // ------------------------------------------------------------
    let mut exec = Executor::new();

    // ------------------------------------------------------------
    // 2️⃣  Spawn the server in a separate OS thread (so the example
    //     can continue to the client code).
    // ------------------------------------------------------------
    thread::spawn(move || {
        // Server handler – echo back a static reply.
        let handler = |mut chan: fusion_lang::network::SecureChannel| -> Result<()> {
            let msg = chan.recv()?;
            println!("Server received: {}", String::from_utf8_lossy(&msg));
            chan.send(b"hello from server")?;
            Ok(())
        };
        // Run the server – this blocks forever, but the thread will be
        // terminated when the main function exits.
        let _ = FusionNetwork::run_server("127.0.0.1:7878", handler, &mut exec);
    });

    // Give the server a moment to start up.
    thread::sleep(Duration::from_millis(200));

    // ------------------------------------------------------------
    // 3️⃣  Client side – connect, send a message, receive reply.
    // ------------------------------------------------------------
    let mut client = FusionNetwork::connect("127.0.0.1:7878")?;
    client.send(b"hello from client")?;
    let reply = client.recv()?;
    println!("Client got: {}", String::from_utf8_lossy(&reply));

    Ok(())
}
