use std::net::UdpSocket;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use anyhow::Result;

pub const MAX_PACKET_SIZE: usize = 1450;
pub const HEADER_SIZE: usize = 16;
pub const PAYLOAD_SIZE: usize = MAX_PACKET_SIZE - HEADER_SIZE;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TensorHeader {
    pub tensor_id: u64,
    pub chunk_index: u32,
    pub total_chunks: u32,
    pub is_parity: u8,
}

pub struct TensorWeaveFabric {
    socket: UdpSocket,
    reassembly_buffer: Arc<Mutex<HashMap<u64, Vec<Option<Vec<u8>>>>>>,
}

impl TensorWeaveFabric {
    pub fn bind(addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(false)?;
        Ok(Self {
            socket,
            reassembly_buffer: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn transmit_tensor(&self, target: &str, tensor_id: u64, data: &[u8]) -> Result<()> {
        let chunks = data.chunks(PAYLOAD_SIZE).collect::<Vec<&[u8]>>();
        let total_chunks = chunks.len();
        
        // Linear payload chunk transmission
        for (idx, chunk) in chunks.iter().enumerate() {
            let header = TensorHeader {
                tensor_id,
                chunk_index: idx as u32,
                total_chunks: total_chunks as u32,
                is_parity: 0,
            };
            let mut packet = bincode::serialize(&header)?;
            packet.extend_from_slice(chunk);
            self.socket.send_to(&packet, target)?;
        }

        // Forward Error Correction: Simple XOR Parity Generation
        if total_chunks > 0 {
            let mut parity_payload = vec![0u8; PAYLOAD_SIZE];
            for chunk in &chunks {
                for (i, byte) in chunk.iter().enumerate() {
                    parity_payload[i] ^= byte;
                }
            }
            let parity_header = TensorHeader {
                tensor_id,
                chunk_index: total_chunks as u32,
                total_chunks: total_chunks as u32,
                is_parity: 1,
            };
            let mut parity_packet = bincode::serialize(&parity_header)?;
            parity_packet.extend_from_slice(&parity_payload);
            self.socket.send_to(&parity_packet, target)?;
        }

        Ok(())
    }

    pub fn receive_and_reconstruct(&self) -> Result<(u64, Vec<u8>)> {
        let mut buf = vec![0u8; MAX_PACKET_SIZE];
        loop {
            let (amt, _) = self.socket.recv_from(&mut buf)?;
            if amt < HEADER_SIZE { continue; }

            let header: TensorHeader = bincode::deserialize(&buf[..HEADER_SIZE])?;
            let mut lock = self.reassembly_buffer.lock();
            
            let entry = lock.entry(header.tensor_id).or_insert_with(|| {
                vec![None; (header.total_chunks + 1) as usize]
            });

            let payload = buf[HEADER_SIZE..amt].to_vec();
            entry[header.chunk_index as usize] = Some(payload);

            // Count available structural pieces
            let received_count = entry.iter().filter(|x| x.is_some()).count();
            let total_expected = header.total_chunks as usize;

            if received_count >= total_expected {
                let mut reassembled_data = Vec::new();
                let mut missing_idx: Option<usize> = None;

                for idx in 0..total_expected {
                    if entry[idx].is_none() {
                        missing_idx = Some(idx);
                    }
                }

                // Packet Erasure Coding Recovery Phase
                if let Some(lost_pos) = missing_idx {
                    if entry[total_expected].is_some() { // Parity packet is present
                        let mut recovered = vec![0u8; PAYLOAD_SIZE];
                        for idx in 0..=total_expected {
                            if idx != lost_pos {
                                if let Some(ref chunk_data) = entry[idx] {
                                    for (i, byte) in chunk_data.iter().enumerate() {
                                        recovered[i] ^= byte;
                                    }
                                }
                            }
                        }
                        entry[lost_pos] = Some(recovered);
                    } else {
                        continue; // Cannot recover without the parity segment
                    }
                }

                for idx in 0..total_expected {
                    if let Some(ref clear_data) = entry[idx] {
                        reassembled_data.extend_from_slice(clear_data);
                    }
                }

                lock.remove(&header.tensor_id);
                return Ok((header.tensor_id, reassembled_data));
            }
        }
    }
}