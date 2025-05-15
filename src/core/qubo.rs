use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::Vtype;

pub struct Qubo<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub name: String,
    pub vtype: Vtype,
    pub matrix_flat: Vec<Bias>,
    pub num_variables: Index,
    pub offset: Bias,
    pub variable_names: Vec<String>,
}
