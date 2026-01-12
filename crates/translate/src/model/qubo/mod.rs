use lunamodel_types::{Bias, Sense, Vtype};

mod back_translate;
mod translate;

pub struct Qubo {
    pub sense: Sense,
    pub name: String,
    pub vtype: Vtype,
    pub matrix_flat: Vec<Bias>,
    pub num_variables: usize,
    pub offset: Bias,
    pub variable_names: Vec<String>,
}

pub struct QuboTranslator;
