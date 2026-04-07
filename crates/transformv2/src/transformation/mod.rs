mod binary_spin;
mod change_sense;
mod eq_constr_to_quad_pen;
mod ge_to_le;
mod integer_to_binary;

pub use binary_spin::BinarySpinPass;
pub use change_sense::ChangeSensePass;
pub use eq_constr_to_quad_pen::EqualityConstraintsToQuadraticPenaltyPass;
pub use ge_to_le::GeToLeConstraintsPass;
pub use integer_to_binary::IntegerToBinaryPass;
