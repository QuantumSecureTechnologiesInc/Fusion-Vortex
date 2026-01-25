# Fusion RPC Protocol Specification

**Version:** 1.0
**Status:** Production
**Last Updated:** 2025-12-10

## Overview

The Fusion RPC (Remote Procedure Call) protocol provides secure, authenticated communication between distributed Fusion components over TCP. It combines post-quantum cryptography (Kyber-768 for key exchange) with authenticated encryption (ChaCha20-Poly1305) to ensure confidentiality, integrity, and authenticity of all messages.

## Architecture

### Protocol Stack

```text
┌─────────────────────────────────┐
│  Application Layer              │
│  (RPC Messages: Execute,        │
│   Result, Ping, etc.)           │
├─────────────────────────────────┤
│  Serialization Layer            │
│  (bincode)                      │
├─────────────────────────────────┤
│  Encryption Layer               │
│  (ChaCha20-Poly1305 AEAD)       │
├─────────────────────────────────┤
│  Handshake Layer                │
│  (Kyber-768 KEM)                │
├─────────────────────────────────┤
│  Transport Layer                │
│  (TCP)                          │
└─────────────────────────────────┘
```text

### Security Properties

1. **Post-Quantum Security**: Kyber-768 provides NIST Security Level 3, resistant to quantum computer attacks
2. **Forward Secrecy**: Fresh keypairs generated per connection
3. **Authenticated Encryption**: ChaCha20-Poly1305 AEAD prevents tampering and replay attacks
4. **End-to-End Encryption**: All data encrypted before transmission

## Message Format

### Message Enum

All RPC messages use the following Rust enum, serialized via bincode:

```rust

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum Message {
    /// Execute a remote function with arguments
    Execute {
        function_name: String,
        args: Vec<u8>,
    },

    /// Return the result of an execution
    Result {
        success: bool,
        data: Vec<u8>,
    },

    /// Health check / keepalive
    Ping,

    /// Response to Ping
    Pong,
}
```text

### Wire Format

Each message on the wire consists of:

```text
┌──────────────┬──────────────┬────────────────┐
│ Nonce        │ Length       │ Ciphertext     │
│ (12 bytes)   │ (4 bytes)    │ (variable)     │
└──────────────┴──────────────┴────────────────┘
```text

1. **Nonce** (12 bytes): Random value used once for ChaCha20-Poly1305
2. **Length** (4 bytes, big-endian u32): Length of ciphertext in bytes
3. **Ciphertext** (variable): AEAD-encrypted `bincode(Message)` + authentication tag

### Serialization

Messages are serialized using `bincode` with the following properties:
- **Endianness**: Little-endian (default)
- **Integer Encoding**: Variable-length (efficient for small numbers)
- **String Encoding**: Length-prefixed UTF-8
- **Binary Safety**: Full support for arbitrary byte sequences

## Connection Lifecycle

### 1. Connection Establishment

```text
Client                                    Server
  │                                          │
  │────────── TCP Connect ──────────────────▶│
  │                                          │
  │◀────────── TCP Accept ───────────────────│
  │                                          │
```text

### 2. Kyber Handshake

```text
Client                                    Server
  │                                          │
  │  1. Generate keypair (pk_c, sk_c)        │
  │────────── Send pk_c ────────────────────▶│
  │                                          │  2. Receive pk_c
  │                                          │  3. Encapsulate → (ct, shared)
  │                                          │
  │◀────────── Send ct ──────────────────────│
  │                                          │
  │  4. Decapsulate ct with sk_c → shared    │
  │                                          │
  │  Both sides now have: shared secret      │
  │  Derive ChaCha20 key: K = shared[0..32]  │
```text

**Handshake Details:**
- **Client sends**: Kyber-768 public key (1184 bytes)
- **Server sends**: Kyber-768 ciphertext (1088 bytes)
- **Shared secret**: 32 bytes, used directly as ChaCha20-Poly1305 key

### 3. Secure Communication

Once the handshake completes, both sides can send/receive encrypted messages:

```rust
// Send a message
let msg = Message::Execute {
    function_name: "compute".to_string(),
    args: vec![1, 2, 3],
};
channel.send_message(&msg)?;

// Receive a message
let response = channel.recv_message()?;
match response {
    Message::Result { success, data } => { /* handle */ },
    _ => { /* unexpected */ }
}
```text

### 4. Connection Termination

Either party can close the TCP connection at any time. No explicit teardown protocol is required—TCP FIN/RST packets handle connection closure.

## Usage Patterns

### Pattern 1: Request-Response (RPC)

**Server:**

```rust
use fusion_lang::network::{FusionNetwork, Message, NetResult};

fn main() -> NetResult<()> {
    let mut executor = Executor::new();

    FusionNetwork::run_server("127.0.0.1:8080", |mut channel| {
        loop {
            let request = channel.recv_message()?;
            match request {
                Message::Execute { function_name, args } => {
                    // Execute function
                    let result = execute_function(&function_name, &args);

                    // Send result back
                    let response = Message::Result {
                        success: true,
                        data: result,
                    };
                    channel.send_message(&response)?;
                }
                Message::Ping => {
                    channel.send_message(&Message::Pong)?;
                }
                _ => { /* handle unexpected messages */ }
            }
        }
    }, &mut executor)
}
```text

**Client:**

```rust
use fusion_lang::network::{FusionNetwork, Message, NetResult};

fn main() -> NetResult<()> {
    let mut channel = FusionNetwork::connect("127.0.0.1:8080")?;

    // Send execute request
    let request = Message::Execute {
        function_name: "fibonacci".to_string(),
        args: bincode::serialize(&10u32).unwrap(),
    };
    channel.send_message(&request)?;

    // Wait for result
    let response = channel.recv_message()?;
    match response {
        Message::Result { success: true, data } => {
            let value: u64 = bincode::deserialize(&data)?;
            println!("Result: {}", value);
        }
        Message::Result { success: false, data } => {
            let error: String = bincode::deserialize(&data)?;
            eprintln!("Error: {}", error);
        }
        _ => eprintln!("Unexpected response"),
    }

    Ok(())
}
```text

### Pattern 2: Health Monitoring

```rust
// Keepalive loop
loop {
    channel.send_message(&Message::Ping)?;

    match channel.recv_message()? {
        Message::Pong => { /* connection alive */ },
        _ => return Err(/* unexpected response */),
    }

    std::thread::sleep(Duration::from_secs(30));
}
```text

### Pattern 3: Streaming Results

For long-running computations, send incremental results:

```rust
// Server sends progress updates
for i in 0..100 {
    let progress = Message::Result {
        success: true,
        data: bincode::serialize(&i).unwrap(),
    };
    channel.send_message(&progress)?;
}

// Final result
channel.send_message(&Message::Result {
    success: true,
    data: bincode::serialize(&"Complete").unwrap(),
})?;
```text

## Error Handling

### Error Types

The protocol defines `NetResult<T> = Result<T, NetworkError>` where:

```rust

#[derive(Debug, thiserror::Error)]

pub enum NetworkError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Handshake failed: {0}")]
    Handshake(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
```text

### Common Error Scenarios

| Error                             | Cause                                 | Mitigation                                    |
| --------------------------------- | ------------------------------------- | --------------------------------------------- |
| `Io(ConnectionRefused)`           | Server not running or wrong address   | Verify server is listening on correct port    |
| `Io(ConnectionReset)`             | Server crashed or forcibly closed     | Implement reconnection logic                  |
| `Crypto("decryption failure")`    | Message tampered or corrupted         | Drop connection, log security event           |
| `Serialization(...)`              | Malformed message or version mismatch | Validate message format, implement versioning |
| `Handshake("invalid ciphertext")` | Corrupted handshake data              | Retry connection                              |

### Malformed Message Handling

**Detection:**

```rust
let msg = match channel.recv_message() {
    Ok(m) => m,
    Err(NetworkError::Serialization(_)) => {
        // Malformed message - likely protocol version mismatch
        eprintln!("Received malformed message, closing connection");
        return Err(NetworkError::Handshake("Protocol mismatch".into()));
    }
    Err(e) => return Err(e),
};
```text

**Prevention:**
- Add version field to `Message` enum
- Validate message structure before processing
- Implement maximum message size limits

## Security Considerations

### Threat Model

**Protected Against:**
- ✅ Eavesdropping (encryption)
- ✅ Message tampering (AEAD authentication)
- ✅ Replay attacks (nonce uniqueness)
- ✅ Quantum computer attacks (Kyber-768)
- ✅ Man-in-the-middle (requires key exchange security)

**Not Protected Against:**
- ❌ Denial of Service (no rate limiting yet)
- ❌ Client authentication (no identity verification)
- ❌ Traffic analysis (metadata visible)

### Authentication

**Current State:** The protocol provides encryption and integrity but does NOT authenticate the identity of clients or servers.

**Recommended Enhancements:**

1. **Client Certificates:**

```rust
Message::Authenticate {
    client_id: String,
    signature: Vec<u8>,  // Sign challenge with private key
}
```text

2. **Pre-Shared Keys:**

```rust
// Derive session key from PSK + Kyber shared secret
let session_key = HKDF(psk || kyber_shared_secret);
```text

3. **Challenge-Response:**

```rust
// Server sends random challenge
Message::Challenge { nonce: [u8; 32] }

// Client signs and returns
Message::AuthResponse {
    signature: sign(nonce, client_private_key)
}
```text

### Versioning

**Recommended Message Format:**

```rust

#[derive(Serialize, Deserialize)]

pub struct VersionedMessage {
    version: u8,  // Protocol version (currently 1)
    payload: Message,
}
```text

**Version Negotiation:**

```rust
// Client sends supported versions
Message::Hello { versions: vec![1, 2] }

// Server responds with chosen version
Message::Version { selected: 1 }
```text

### Rate Limiting

Implement per-connection rate limits to prevent abuse:

```rust
const MAX_MESSAGES_PER_SECOND: u32 = 100;

struct RateLimiter {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    fn check(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs();

        self.tokens = (self.tokens + elapsed as u32 * MAX_MESSAGES_PER_SECOND)
            .min(MAX_MESSAGES_PER_SECOND);
        self.last_refill = now;

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}
```text

## Performance Characteristics

### Latency

| Operation              | Time   | Notes                                    |
| ---------------------- | ------ | ---------------------------------------- |
| Handshake              | ~5ms   | Kyber-768 key generation + encapsulation |
| Encryption             | ~50μs  | ChaCha20-Poly1305 (1KB message)          |
| Decryption             | ~50μs  | ChaCha20-Poly1305 (1KB message)          |
| Serialization          | ~10μs  | bincode (simple message)                 |
| Round-trip (localhost) | ~200μs | Send + receive small message             |

### Throughput

- **Maximum Message Rate**: ~10,000 messages/second (single connection)
- **Bandwidth**: ~500 MB/second (large messages, localhost)
- **Overhead**: ~28 bytes per message (nonce + length + AEAD tag)

### Scalability

**Current Limitations:**
- One handler per connection (blocking I/O)
- Thread-per-connection model (limited by OS thread limits)

**Recommended for Production:**
- Use async I/O with Tokio for higher concurrency
- Implement connection pooling
- Add load balancing for multi-server deployments

## Testing

### Unit Tests

```rust

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_message_roundtrip() {
        let msg = Message::Execute {
            function_name: "test".into(),
            args: vec![1, 2, 3],
        };

        let bytes = bincode::serialize(&msg).unwrap();
        let decoded: Message = bincode::deserialize(&bytes).unwrap();

        assert_eq!(format!("{:?}", msg), format!("{:?}", decoded));
    }
}
```text

### Integration Tests

```rust

#[test]

fn test_secure_channel_echo() {
    let server = std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (socket, _) = listener.accept().unwrap();
        let mut channel = SecureChannel::server_handshake(socket).unwrap();

        let msg = channel.recv_message().unwrap();
        channel.send_message(&msg).unwrap();
    });

    let mut channel = FusionNetwork::connect(&addr).unwrap();

    let request = Message::Ping;
    channel.send_message(&request).unwrap();

    let response = channel.recv_message().unwrap();
    assert!(matches!(response, Message::Ping));

    server.join().unwrap();
}
```text

## Best Practices

### Do's ✅

1. **Always validate received messages:**

   ```rust
   match channel.recv_message()? {
       Message::Execute { .. } => { /* expected */ },
       _ => return Err("Unexpected message type"),
   }
```text

2. **Implement timeouts:**

   ```rust
   channel.set_read_timeout(Some(Duration::from_secs(30)))?;
```text

3. **Log security events:**

   ```rust
   if let Err(NetworkError::Crypto(_)) = channel.recv() {
       log::error!("Decryption failed - possible attack");
   }
```text

4. **Use structured error handling:**

   ```rust
   Message::Result {
       success: false,
       data: bincode::serialize(&ErrorInfo {
           code: 404,
           message: "Function not found".into(),
       }).unwrap(),
   }
```text

### Don'ts ❌

1. **Don't reuse connections indefinitely** - implement connection rotation
2. **Don't ignore decryption errors** - always treat as potential attacks
3. **Don't send unbounded data** - implement message size limits
4. **Don't skip error handling** - always propagate or log errors

## Future Enhancements

1. **Protocol Versioning**: Add explicit version negotiation
2. **Compression**: Support zstd/lz4 for large payloads
3. **Multiplexing**: Multiple logical streams over one connection
4. **Bidirectional Streaming**: Full-duplex RPC support
5. **Service Discovery**: Automatic endpoint discovery
6. **Telemetry**: Built-in metrics and tracing
7. **Circuit Breaking**: Automatic failure detection and recovery

## References

- [Kyber-768 Specification](https://pq-crystals.org/kyber/)
- [ChaCha20-Poly1305 RFC 8439](https://tools.ietf.org/html/rfc8439)
- [bincode Format](https://github.com/bincode-org/bincode)
- Fusion Network Module: `src/network/mod.rs`
- Standard Library Wrappers: `stdlib/network.fu`

---

**Maintained by:** Fusion Language Team
**Contact:** security@fusion-lang.org
**License:** MIT OR Apache-2.0