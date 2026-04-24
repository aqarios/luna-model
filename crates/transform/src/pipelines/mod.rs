//! Turnkey pipelines composed from the built-in passes.
mod to_binary_minimization;
mod to_unconstrained_binary;

pub use to_binary_minimization::ToBinaryMinimizationPipeline;
pub use to_unconstrained_binary::ToUnconstrainedBinaryPipeline;
