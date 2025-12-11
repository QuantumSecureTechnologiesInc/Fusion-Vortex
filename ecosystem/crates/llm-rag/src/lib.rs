/// Production Vector Store for RAG.
///
/// Features:
/// - Thread-safe storage (Arc<RwLock>) allowing concurrent reads.
/// - Robust math (Cosine Similarity with zero-magnitude checks).
/// - Typed Errors.
use fusion_core::types::tensor::Vector1D;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RagError {
    #[error("Dimension mismatch: Document has {0}, Store expects {1}")]
    DimensionMismatch(usize, usize),
    #[error("Mathematical error: {0}")]
    MathError(String),
    #[error("Lock poisoned")]
    LockError,
}

pub type RagResult<T> = Result<T, RagError>;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub embedding: Vector1D<f64>,
    pub metadata: HashMap<String, String>,
}

/// Thread-safe Vector Database.
/// Optimized for high-read / low-write workloads (typical for RAG).
pub struct VectorStore {
    // Inner state protected by RwLock for concurrent readers (search) / exclusive writer (add)
    docs: Arc<RwLock<Vec<Document>>>,
    embed_dim: usize,
}

impl VectorStore {
    pub fn new(embed_dim: usize) -> Self {
        Self {
            docs: Arc::new(RwLock::new(Vec::new())),
            embed_dim,
        }
    }

    /// Add a document transactionally.
    /// Returns error if dimension doesn't match store config.
    pub fn add(&self, doc: Document) -> RagResult<()> {
        if doc.embedding.shape[0] != self.embed_dim {
            return Err(RagError::DimensionMismatch(
                doc.embedding.shape[0],
                self.embed_dim,
            ));
        }

        let mut write_guard = self.docs.write().map_err(|_| RagError::LockError)?;
        write_guard.push(doc);
        Ok(())
    }

    /// Search for top-k similar documents.
    /// Safe against concurrent writes.
    pub fn search(&self, query_vec: &Vector1D<f64>, k: usize) -> RagResult<Vec<(f64, Document)>> {
        if query_vec.shape[0] != self.embed_dim {
            return Err(RagError::DimensionMismatch(
                query_vec.shape[0],
                self.embed_dim,
            ));
        }

        let read_guard = self.docs.read().map_err(|_| RagError::LockError)?;

        let mut scores = Vec::with_capacity(read_guard.len());

        for doc in read_guard.iter() {
            let sim = cosine_similarity(&doc.embedding, query_vec)?;
            scores.push((sim, doc.clone()));
        }

        // Sort descending by score. Handle NaN via unwrap_or to prevent panic in sort.
        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        if k < scores.len() {
            scores.truncate(k);
        }

        Ok(scores)
    }
}

/// Robust Cosine Similarity.
/// sim(A, B) = (A . B) / (||A|| * ||B||)
fn cosine_similarity(a: &Vector1D<f64>, b: &Vector1D<f64>) -> RagResult<f64> {
    let mut dot = 0.0;
    let mut norm_a_sq = 0.0;
    let mut norm_b_sq = 0.0;

    for i in 0..a.shape[0] {
        let va = a
            .get([i])
            .map_err(|_| RagError::MathError("Index Error".into()))?;
        let vb = b
            .get([i])
            .map_err(|_| RagError::MathError("Index Error".into()))?;

        dot += va * vb;
        norm_a_sq += va * va;
        norm_b_sq += vb * vb;
    }

    let norm_a = norm_a_sq.sqrt();
    let norm_b = norm_b_sq.sqrt();

    // Prevent Division by Zero
    if norm_a < 1e-9 || norm_b < 1e-9 {
        return Err(RagError::MathError(
            "Vector magnitude too close to zero".into(),
        ));
    }

    Ok(dot / (norm_a * norm_b))
}
