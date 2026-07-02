//! Public translation-target enum definitions.

use strum_macros::Display;

/// Canonical model translation targets supported by LunaModel.
#[derive(Debug, Display, Hash, PartialEq)]
pub enum TranslationTarget {
    /// Quadratic unconstrained binary optimization representation.
    Qubo,
    /// LP file representation.
    Lp,
    /// MPS file representation.
    Mps,
    /// D-Wave style binary quadratic model representation.
    Bqm,
    /// Constrained quadratic model representation.
    Cqm,
    /// Qiskit OptimizationProblem representation.
    OptMapper,
}
