use crate::core::{Model, VarId};

pub struct MatrixTranslator {}

impl MatrixTranslator {
    pub fn model_from_dense(name: Option<String>, dense: &[f64], num_variables: VarId) -> Model {
        Model::new_from_dense(name, dense, num_variables)
    }
}
