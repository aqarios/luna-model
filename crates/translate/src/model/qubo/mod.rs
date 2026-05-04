//! QUBO-specific translator types and helpers.

use lunamodel_types::{Bias, Sense, Vtype};

mod back_translate;
mod translate;

/// Dense flattened QUBO representation used by the translator APIs.
pub struct Qubo {
    /// Optimization sense associated with the QUBO.
    pub sense: Sense,
    /// Problem name.
    pub name: String,
    /// Variable type used by the QUBO variables.
    pub vtype: Vtype,
    /// Row-major flattened QUBO matrix.
    pub matrix_flat: Vec<Bias>,
    /// Number of variables represented by the matrix.
    pub num_variables: usize,
    /// Constant offset term outside the matrix.
    pub offset: Bias,
    /// Optional variable names aligned with matrix order.
    pub variable_names: Vec<String>,
}

/// Entry type for QUBO translation operations.
pub struct QuboTranslator;
