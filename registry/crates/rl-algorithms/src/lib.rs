/// Production Reinforcement Learning Algorithms.
/// 
/// Implements GAE (Generalized Advantage Estimation) for stable policy gradient updates.

use fusion_core::types::tensor::{Vector1D, Tensor};
use fusion_core::traits::Numeric;
use fusion_core::FusionResult;

pub struct GAECalculator {
   gamma: f64, // Discount factor
   lambda: f64, // Smoothing parameter
}

impl GAECalculator {
   pub fn new(gamma: f64, lambda: f64) -> Self {
       Self { gamma, lambda }
   }

   /// Compute Advantages using GAE.
   /// A_t = delta_t + (gamma * lambda) * A_{t+1}
   /// delta_t = r_t + gamma * V(s_{t+1}) - V(s_t)
   pub fn compute_advantages(
       &self,
       rewards: &Vector1D<f64>,
       values: &Vector1D<f64>,
       next_val: f64,
       dones: &Vector1D<f64>, // 1.0 if done, 0.0 else
   ) -> FusionResult<Vector1D<f64>> {
       let len = rewards.shape[0];
       if values.shape[0] != len || dones.shape[0] != len {
           return Err(fusion_core::FusionError::ShapeMismatch {
               op: "GAE".into(),
               lhs: vec![len],
               rhs: vec![values.shape[0], dones.shape[0]],
           });
       }

       let mut advantages = vec![0.0; len];
       let mut last_gae_lam = 0.0;

       for t in (0..len).rev() {
           let r_t = rewards.get([t])?;
           let v_t = values.get([t])?;
           let done = dones.get([t])?;
           
           let next_v = if t == len - 1 {
               next_val
           } else {
               values.get([t + 1])?
           };

           // If done, next state value is 0 for the purpose of this trajectory
           let mask = 1.0 - done; 
           
           // Temporal Difference Error
           let delta = r_t + self.gamma * next_v * mask - v_t;
           
           last_gae_lam = delta + self.gamma * self.lambda * mask * last_gae_lam;
           advantages[t] = last_gae_lam;
       }

       Tensor::new(advantages, [len])
   }
}