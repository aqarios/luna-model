use core::fmt;

use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    ExpressionBase, Model, Vtype,
};

#[derive(Debug, Clone)]
pub struct ModelNotQuadraticError;
impl std::error::Error for ModelNotQuadraticError {}
impl fmt::Display for ModelNotQuadraticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the model is not linear or quadratic, thus cannot be translated to a matrix."
        )
    }
}

pub struct MatrixTranslator {}

impl MatrixTranslator {
    pub fn model_from_dense<Index, Bias>(
        name: Option<String>,
        dense: &[Bias],
        num_variables: Index,
        vtype: Vtype,
    ) -> Model<Index, Bias>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        Model::new_from_dense(name, dense, num_variables, vtype)
    }

    // pub fn model_to_dense<Index, Bias>(
    //     model: Model<Index, Bias>,
    // ) -> Result<Vec<Bias>, ModelNotQuadraticError>
    // where
    //     Index: IndexConstraints,
    //     Bias: BiasConstraints,
    // {
    //     if model.objective.borrow().has_higher_order() {
    //         return Err(ModelNotQuadraticError);
    //     }

    //     let nvars = model.objective.borrow().num_variables();
    //     let flat: Vec<Bias> = Vec::with_capacity(nvars * nvars);

    //     for (u, bias) in model.objective.borrow().linear.iter() {}

    //     for (u, v, bias) in model.objective.borrow().quadratic.iter_flat() {}

    //     Ok(flat)
    // }
}
