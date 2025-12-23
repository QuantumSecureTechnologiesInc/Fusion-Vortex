use fusion_std::error::{StdError, StdResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Production Telemetry Ingestor
/// Receives metrics, traces, and logs from distributed systems

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryType {
    Metric,
    Trace,
    Log,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub event_type: TelemetryType,
    pub timestamp: i64,
    pub source: String,
    pub data: HashMap<String, String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TelemetryStats {
    pub total_events: u64,
    pub metrics_count: u64,
    pub traces_count: u64,
    pub logs_count: u64,
    pub bytes_received: u64,
}

pub struct TelemetryIngestor {
    bind_addr: String,
    events: Arc<Mutex<Vec<TelemetryEvent>>>,
    stats: Arc<Mutex<TelemetryStats>>,
    max_events: usize,
}

impl TelemetryIngestor {
    pub fn new(bind_addr: String) -> Self {
        Self {
            bind_addr,
            events: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(TelemetryStats {
                total_events: 0,
                metrics_count: 0,
                traces_count: 0,
                logs_count: 0,
                bytes_received: 0,
            })),
            max_events: 10000,
        }
    }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    /// Start the telemetry ingestor server
    pub async fn start(&self) -> StdResult<()> {
        let listener = TcpListener::bind(&self.bind_addr).await.map_err(|e| {
            StdError::IoError(format!("Failed to bind to {}: {}", self.bind_addr, e))
        })?;

        println!("Telemetry Ingestor listening on {}", self.bind_addr);

        loop {
            let (mut socket, addr) = listener
                .accept()
                .await
                .map_err(|e| StdError::IoError(e.to_string()))?;

            let events = Arc::clone(&self.events);
            let stats = Arc::clone(&self.stats);
            let max_events = self.max_events;

            tokio::spawn(async move {
                let mut buffer = vec![0u8; 8192];

                loop {
                    match socket.read(&mut buffer).await {
                        Ok(0) => break, // Connection closed
                        Ok(n) => {
                            // Parse telemetry event
                            if let Ok(event) = Self::parse_event(&buffer[..n]) {
                                {
                                    let mut events_lock = events.lock().unwrap();
                                    let mut stats_lock = stats.lock().unwrap();

                                    // Update stats
                                    stats_lock.total_events += 1;
                                    stats_lock.bytes_received += n as u64;

                                    match event.event_type {
                                        TelemetryType::Metric => stats_lock.metrics_count += 1,
                                        TelemetryType::Trace => stats_lock.traces_count += 1,
                                        TelemetryType::Log => stats_lock.logs_count += 1,
                                    }

                                    // Store event (with circular buffer)
                                    if events_lock.len() >= max_events {
                                        events_lock.remove(0);
                                    }
                                    events_lock.push(event);
                                } // Locks dropped here

                                // Send acknowledgment
                                let _ = socket.write_all(b"ACK\n").await;
                            } else {
                                let _ = socket.write_all(b"ERR: Invalid format\n").await;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading from {}: {}", addr, e);
                            break;
                        }
                    }
                }
            });
        }
    }

    /// Parse telemetry event from bytes
    fn parse_event(data: &[u8]) -> StdResult<TelemetryEvent> {
        // Try JSON parsing first
        if let Ok(event) = serde_json::from_slice::<TelemetryEvent>(data) {
            return Ok(event);
        }

        // Fallback: simple line protocol parsing
        // Format: TYPE|timestamp|source|key1=val1,key2=val2
        let text = std::str::from_utf8(data).map_err(|e| StdError::InvalidInput(e.to_string()))?;

        let parts: Vec<&str> = text.trim().split('|').collect();
        if parts.len() < 3 {
            return Err(StdError::InvalidInput("Invalid telemetry format".into()));
        }

        let event_type = match parts[0] {
            "METRIC" => TelemetryType::Metric,
            "TRACE" => TelemetryType::Trace,
            "LOG" => TelemetryType::Log,
            _ => return Err(StdError::InvalidInput("Unknown event type".into())),
        };

        let timestamp = parts[1]
            .parse::<i64>()
            .map_err(|e| StdError::InvalidInput(e.to_string()))?;

        let source = parts[2].to_string();

        let mut data_map = HashMap::new();
        if parts.len() > 3 {
            for kv in parts[3].split(',') {
                if let Some((k, v)) = kv.split_once('=') {
                    data_map.insert(k.to_string(), v.to_string());
                }
            }
        }

        Ok(TelemetryEvent {
            event_type,
            timestamp,
            source,
            data: data_map,
            tags: HashMap::new(),
        })
    }

    /// Get current statistics
    pub fn get_stats(&self) -> TelemetryStats {
        self.stats.lock().unwrap().clone()
    }

    /// Query events by type
    pub fn query_events(
        &self,
        event_type: Option<TelemetryType>,
        limit: usize,
    ) -> Vec<TelemetryEvent> {
        let events = self.events.lock().unwrap();

        events
            .iter()
            .filter(|e| {
                if let Some(ref filter_type) = event_type {
                    std::mem::discriminant(&e.event_type) == std::mem::discriminant(filter_type)
                } else {
                    true
                }
            })
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear all stored events
    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }

    /// Export events to JSON
    pub fn export_json(&self) -> StdResult<String> {
        let events = self.events.lock().unwrap();
        serde_json::to_string_pretty(&*events).map_err(|e| StdError::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_event() {
        let json =
            r#"{"event_type":"Metric","timestamp":1234567890,"source":"test","data":{},"tags":{}}"#;
        let event = TelemetryIngestor::parse_event(json.as_bytes()).unwrap();
        assert_eq!(event.timestamp, 1234567890);
        assert_eq!(event.source, "test");
    }

    #[test]
    fn test_parse_line_protocol() {
        let line = "METRIC|1234567890|test-service|cpu=50,mem=1024";
        let event = TelemetryIngestor::parse_event(line.as_bytes()).unwrap();

        assert_eq!(event.timestamp, 1234567890);
        assert_eq!(event.source, "test-service");
        assert_eq!(event.data.get("cpu"), Some(&"50".to_string()));
        assert_eq!(event.data.get("mem"), Some(&"1024".to_string()));
    }

    #[test]
    fn test_stats_tracking() {
        let ingestor = TelemetryIngestor::new("127.0.0.1:9090".to_string());
        let stats = ingestor.get_stats();
        assert_eq!(stats.total_events, 0);
    }
}
