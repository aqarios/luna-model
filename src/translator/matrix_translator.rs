use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    Model,
};

pub struct MatrixTranslator {}

impl MatrixTranslator {
    pub fn model_from_dense<Index, Bias>(
        name: Option<String>,
        dense: &[Bias],
        num_variables: Index,
    ) -> Model<Index, Bias>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        Model::new_from_dense(name, dense, num_variables)
    }
}
