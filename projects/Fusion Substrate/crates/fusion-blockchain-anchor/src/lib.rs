use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use thiserror::Error;

/// Blockchain anchor errors
#[derive(Debug, Error)]
pub enum AnchorError {
    #[error("Anchor creation failed: {0}")]
    CreationFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Anchor not found: {0}")]
    NotFound(String),

    #[error("Invalid anchor format: {0}")]
    InvalidFormat(String),
}

/// Blockchain network type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    Ethereum,
    Polygon,
    Cosmos,
    Custom,
}

/// Anchor record on blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorRecord {
    /// Unique identifier
    pub id: String,
    /// Hash of the data being anchored
    pub data_hash: String,
    /// Block number where anchored
    pub block_number: u64,
    /// Transaction hash
    pub transaction_hash: String,
    /// Network where anchored
    pub network: BlockchainNetwork,
    /// Timestamp
    pub timestamp: u64,
}

/// Merkle root for batch anchoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleRoot {
    pub root_hash: String,
    pub leaves: Vec<String>,
}

impl MerkleRoot {
    /// Create a Merkle root from a list of data hashes
    pub fn from_hashes(hashes: Vec<String>) -> Self {
        if hashes.is_empty() {
            return Self {
                root_hash: String::new(),
                leaves: vec![],
            };
        }

        let root = Self::compute_root(&hashes);
        Self {
            root_hash: root,
            leaves: hashes,
        }
    }

    fn compute_root(hashes: &[String]) -> String {
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        let mut current_level = hashes.to_vec();

        while current_level.len() > 1 {
            let mut next_level = vec![];

            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    let mut hasher = Sha3_256::new();
                    hasher.update(chunk[0].as_bytes());
                    hasher.update(chunk[1].as_bytes());
                    hex::encode(hasher.finalize())
                } else {
                    chunk[0].clone()
                };
                next_level.push(combined);
            }

            current_level = next_level;
        }

        current_level[0].clone()
    }

    /// Verify a hash is included in the Merkle tree
    pub fn verify_inclusion(&self, hash: &str) -> bool {
        self.leaves.contains(&hash.to_string())
    }
}

/// Blockchain anchor for immutable audit trails
///
/// In production, this would integrate with actual blockchain networks:
/// - Ethereum via web3
/// - Cosmos via CosmWasm
/// - Custom chains via RPC
///
/// This implementation provides the anchoring API with simulated blockchain interaction
pub struct BlockchainAnchor {
    anchors: HashMap<String, AnchorRecord>,
    next_block: u64,
    network: BlockchainNetwork,
}

impl BlockchainAnchor {
    /// Create a new blockchain anchor
    pub fn new(network: BlockchainNetwork) -> Self {
        Self {
            anchors: HashMap::new(),
            next_block: 1,
            network,
        }
    }

    /// Anchor a single data hash to the blockchain
    pub fn anchor(&mut self, id: String, data: &[u8]) -> Result<AnchorRecord, AnchorError> {
        let data_hash = hex::encode(Sha3_256::digest(data));

        // In production, this would submit a transaction to the blockchain
        let record = AnchorRecord {
            id: id.clone(),
            data_hash,
            block_number: self.next_block,
            transaction_hash: self.generate_tx_hash(),
            network: self.network,
            timestamp: Self::current_timestamp(),
        };

        self.next_block += 1;
        self.anchors.insert(id, record.clone());

        Ok(record)
    }

    /// Anchor multiple data hashes in a single transaction using Merkle tree
    pub fn anchor_batch(
        &mut self,
        data_items: Vec<(String, Vec<u8>)>,
    ) -> Result<Vec<AnchorRecord>, AnchorError> {
        if data_items.is_empty() {
            return Ok(vec![]);
        }

        // Compute hashes
        let hashes: Vec<String> = data_items
            .iter()
            .map(|(_, data)| hex::encode(Sha3_256::digest(data)))
            .collect();

        // Create Merkle root
        let merkle = MerkleRoot::from_hashes(hashes.clone());

        // Create anchor records
        let mut records = vec![];
        let tx_hash = self.generate_tx_hash();
        let block = self.next_block;

        for ((id, _), data_hash) in data_items.into_iter().zip(hashes.into_iter()) {
            let record = AnchorRecord {
                id: id.clone(),
                data_hash,
                block_number: block,
                transaction_hash: tx_hash.clone(),
                network: self.network,
                timestamp: Self::current_timestamp(),
            };

            self.anchors.insert(id, record.clone());
            records.push(record);
        }

        self.next_block += 1;

        Ok(records)
    }

    /// Verify an anchor record
    pub fn verify(&self, id: &str, data: &[u8]) -> Result<(), AnchorError> {
        let record = self
            .anchors
            .get(id)
            .ok_or_else(|| AnchorError::NotFound(id.to_string()))?;

        let computed_hash = hex::encode(Sha3_256::digest(data));

        if record.data_hash != computed_hash {
            return Err(AnchorError::VerificationFailed(format!(
                "Hash mismatch for anchor {}",
                id
            )));
        }

        Ok(())
    }

    /// Get an anchor record by ID
    pub fn get_anchor(&self, id: &str) -> Option<&AnchorRecord> {
        self.anchors.get(id)
    }

    /// List all anchor IDs
    pub fn list_anchors(&self) -> Vec<String> {
        self.anchors.keys().cloned().collect()
    }

    /// Get anchors by block number
    pub fn get_by_block(&self, block_number: u64) -> Vec<&AnchorRecord> {
        self.anchors
            .values()
            .filter(|record| record.block_number == block_number)
            .collect()
    }

    fn generate_tx_hash(&self) -> String {
        let data = format!("{}{}", self.next_block, Self::current_timestamp());
        hex::encode(Sha3_256::digest(data.as_bytes()))
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for BlockchainAnchor {
    fn default() -> Self {
        Self::new(BlockchainNetwork::Custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_anchor() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Ethereum);

        let data = b"important document";
        let record = anchor.anchor("doc1".to_string(), data).unwrap();

        assert_eq!(record.id, "doc1");
        assert_eq!(record.network, BlockchainNetwork::Ethereum);
        assert!(record.block_number > 0);
    }

    #[test]
    fn test_anchor_verification() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Ethereum);

        let data = b"test data";
        anchor.anchor("test1".to_string(), data).unwrap();

        assert!(anchor.verify("test1", data).is_ok());
    }

    #[test]
    fn test_verification_failure() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Ethereum);

        let original_data = b"original";
        anchor.anchor("test1".to_string(), original_data).unwrap();

        let tampered_data = b"tampered";
        assert!(matches!(
            anchor.verify("test1", tampered_data),
            Err(AnchorError::VerificationFailed(_))
        ));
    }

    #[test]
    fn test_batch_anchoring() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Polygon);

        let items = vec![
            ("doc1".to_string(), b"data1".to_vec()),
            ("doc2".to_string(), b"data2".to_vec()),
            ("doc3".to_string(), b"data3".to_vec()),
        ];

        let records = anchor.anchor_batch(items).unwrap();

        assert_eq!(records.len(), 3);
        // All should be in the same block
        assert_eq!(records[0].block_number, records[1].block_number);
        assert_eq!(records[1].block_number, records[2].block_number);
        // All should have the same transaction hash (batch transaction)
        assert_eq!(records[0].transaction_hash, records[1].transaction_hash);
    }

    #[test]
    fn test_get_anchor() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Cosmos);

        let data = b"retrieve test";
        anchor.anchor("test1".to_string(), data).unwrap();

        let record = anchor.get_anchor("test1").unwrap();
        assert_eq!(record.id, "test1");
    }

    #[test]
    fn test_list_anchors() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Custom);

        anchor.anchor("a1".to_string(), b"data1").unwrap();
        anchor.anchor("a2".to_string(), b"data2").unwrap();
        anchor.anchor("a3".to_string(), b"data3").unwrap();

        let ids = anchor.list_anchors();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&"a1".to_string()));
        assert!(ids.contains(&"a2".to_string()));
        assert!(ids.contains(&"a3".to_string()));
    }

    #[test]
    fn test_get_by_block() {
        let mut anchor = BlockchainAnchor::new(BlockchainNetwork::Ethereum);

        // Anchor in block 1
        anchor.anchor("b1_1".to_string(), b"data1").unwrap();
        anchor.anchor("b1_2".to_string(), b"data2").unwrap();

        // Anchor in block 2
        anchor.anchor("b2_1".to_string(), b"data3").unwrap();

        let block1_anchors = anchor.get_by_block(1);
        assert_eq!(block1_anchors.len(), 2);

        let block2_anchors = anchor.get_by_block(2);
        assert_eq!(block2_anchors.len(), 1);
    }

    #[test]
    fn test_merkle_root() {
        let hashes = vec![
            "hash1".to_string(),
            "hash2".to_string(),
            "hash3".to_string(),
        ];

        let merkle = MerkleRoot::from_hashes(hashes.clone());

        assert!(!merkle.root_hash.is_empty());
        assert_eq!(merkle.leaves.len(), 3);
    }

    #[test]
    fn test_merkle_inclusion() {
        let hashes = vec!["hash1".to_string(), "hash2".to_string()];

        let merkle = MerkleRoot::from_hashes(hashes);

        assert!(merkle.verify_inclusion("hash1"));
        assert!(merkle.verify_inclusion("hash2"));
        assert!(!merkle.verify_inclusion("hash3"));
    }
}
