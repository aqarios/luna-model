mod binary_spin;
mod change_sense;
mod eq_constr_to_quad_pen;
mod ge_to_le;
mod integer_to_binary;
mod le_to_eq;

pub use binary_spin::{BinarySpinPass, BinarySpinPassArtifact};
pub use change_sense::{ChangeSensePass, ChangeSensePassArtifact};
pub use eq_constr_to_quad_pen::{
    EqualityConstraintsToQuadraticPenaltyArtifact, EqualityConstraintsToQuadraticPenaltyPass,
};
pub use ge_to_le::{GeToLeConstraintsArtifact, GeToLeConstraintsPass};
pub use integer_to_binary::{IntegerToBinaryArtifact, IntegerToBinaryPass};
pub use le_to_eq::{LeToEqConstraintsArtifact, LeToEqConstraintsPass};

pub fn register_backward() {
    lunamodel_transpiler::register_backward::<BinarySpinPass>();
    lunamodel_transpiler::register_backward::<ChangeSensePass>();
    lunamodel_transpiler::register_backward::<EqualityConstraintsToQuadraticPenaltyPass>();
    lunamodel_transpiler::register_backward::<GeToLeConstraintsPass>();
    lunamodel_transpiler::register_backward::<IntegerToBinaryPass>();
    lunamodel_transpiler::register_backward::<LeToEqConstraintsPass>();
}
