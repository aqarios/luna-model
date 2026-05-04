//! Python wrappers for built-in reversible transformation passes.

mod binary_spin;
mod change_sense;
mod eq_constr_to_quad_pen;
mod ge_to_le;
mod integer_to_binary;
mod le_to_eq;
mod reduce_inv_bin;

pub use binary_spin::{PyBinarySpinPass, PyBinarySpinPassArtifact};
pub use change_sense::{PyChangeSensePass, PyChangeSensePassArtifact};
pub use eq_constr_to_quad_pen::{
    PyEqualityConstraintsToQuadraticPenaltyArtifact, PyEqualityConstraintsToQuadraticPenaltyPass,
};
pub use ge_to_le::{PyArtifact as PyGeToLeConstraintsArtifact, PyGeToLeConstraintsPass};
pub use integer_to_binary::{PyArtifact as PyIntegerToBinaryArtifact, PyIntegerToBinaryPass};
pub use le_to_eq::{PyArtifact as PyLeToEqConstraintsArtifact, PyLeToEqConstraintsPass};
pub use reduce_inv_bin::{PyReduceInvertedBinaryPass, PyReduceInvertedBinaryPassArtifact};
