//! Built-in analyses over [`lunamodel_core::Model`] values.
pub(super) mod check_model_specs;
pub(super) mod max_bias;
pub(super) mod min_val_in_constr;
pub(super) mod specs;

pub use check_model_specs::{CheckModelSpecsAnalysis, Nothing};
pub use max_bias::{MaxBias, MaxBiasAnalysis};
pub use min_val_in_constr::{MinConstraintValues, MinValueForConstraintAnalysis};
pub use specs::SpecsAnalysis;
