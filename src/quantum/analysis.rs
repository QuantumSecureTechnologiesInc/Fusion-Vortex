// src/quantum/analysis.rs - Quantum Statistical Analysis
// Provides tools for analyzing measurement results

use std::collections::HashMap;

/// Analysis of quantum measurement results
pub struct QuantumAnalyzer {
    counts: HashMap<String, usize>,
    total_shots: usize,
}

impl QuantumAnalyzer {
    /// Create new analyzer from measurement counts
    pub fn new(counts: HashMap<String, usize>) -> Self {
        let total = counts.values().sum();
        Self {
            counts,
            total_shots: total,
        }
    }

    /// Calculate probability distribution
    pub fn probabilities(&self) -> HashMap<String, f64> {
        self.counts
            .iter()
            .map(|(state, count)| (state.clone(), *count as f64 / self.total_shots as f64))
            .collect()
    }

    /// Calculate result entropy (Shannon entropy)
    pub fn entropy(&self) -> f64 {
        self.probabilities().values().fold(
            0.0,
            |acc, &p| {
                if p > 0.0 {
                    acc - p * p.log2()
                } else {
                    acc
                }
            },
        )
    }

    /// Get most probable state (mode)
    pub fn most_probable(&self) -> (String, f64) {
        let probs = self.probabilities();
        probs
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap_or(("".to_string(), 0.0))
    }

    /// Print ASCII histogram
    pub fn print_histogram(&self) {
        println!("\nQuantum Result Analysis ({} shots):", self.total_shots);
        // Sort by state string for consistent output
        let mut sorted_states: Vec<_> = self.counts.keys().collect();
        sorted_states.sort();

        for state in sorted_states {
            let count = self.counts.get(state).unwrap();
            let probability = *count as f64 / self.total_shots as f64;
            let bar_len = (probability * 50.0) as usize;
            let bar: String = "█".repeat(bar_len);

            println!(
                "|{}⟩: {:>4} ({:5.1}%) {}",
                state,
                count,
                probability * 100.0,
                bar
            );
        }
        println!("Entropy: {:.4} bits\n", self.entropy());
    }
}
