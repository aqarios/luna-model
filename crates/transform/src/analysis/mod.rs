//! Built-in analyses over [`lunamodel_core::Model`] values.
mod utils;

mod check_infeasible_constraints;
mod check_model_specs;
mod max_bias;
mod min_val_in_constr;
mod specs;

pub use check_infeasible_constraints::{CheckInfeasibleConstraintsAnalysis, Ignore};
pub use check_model_specs::{CheckModelSpecsAnalysis, Nothing};
pub use max_bias::{MaxBias, MaxBiasAnalysis};
pub use min_val_in_constr::{MinConstraintValues, MinValueForConstraintAnalysis};
pub use specs::SpecsAnalysis;
