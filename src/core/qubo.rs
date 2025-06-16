use crate::core::Vtype;
use crate::types::{Bias, VarIndex};

pub struct Qubo {
    pub name: String,
    pub vtype: Vtype,
    pub matrix_flat: Vec<Bias>,
    pub num_variables: VarIndex,
    pub offset: Bias,
    pub variable_names: Vec<String>,
}
