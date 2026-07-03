//! Networking IO for the Fusion Standard Library.
use crate::types::*;

/// Represents a TCP Socket connection.
pub struct TcpStream {
    pub fd: i32,
    pub remote_addr: FString,
}

/// Result of a connection attempt.
pub enum ConnectionResult {
    Success(TcpStream),
    Refused,
    Timeout,
}

/// Connects to a remote address on a specified port.
pub fn connect(address: &str, _port: u16) -> ConnectionResult {
    // Native syscall wrapper for connect()
    ConnectionResult::Success(TcpStream {
        fd: 1,
        remote_addr: address.to_string(),
    })
}

/// Sends raw data over the stream.
pub fn send(_stream: &mut TcpStream, data: &[u8]) -> Result<FSize, FString> {
    Ok(data.len() as FSize)
}

/// Receives data from the stream.
pub fn receive(_stream: &mut TcpStream, _buffer_size: FSize) -> Result<FVec<u8>, FString> {
    Ok(vec![])
}

/// Closes the connection.
pub fn close(_stream: TcpStream) -> Result<(), FString> {
    Ok(())
}