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

/// A translator used to read a Quadratic Unconstrained Binary Optimization (QUBO) problem
/// and create an AQM.
pub struct MatrixTranslator {}

impl MatrixTranslator {
    /// Translates a QUBO to an AQM.
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

    /// Back(translate) an AQM to a QUBO.
    ///
    /// This method is required for interactions with solvers that require the optimization
    /// problem to be expressed in a QUBO. We can use the AQM to define our model and send
    /// the information between workers efficiently. The solving process can then use this function
    /// to express the optimization problem in the expected format.
    pub fn model_to_dense<Index, Bias>(
        model: &Model<Index, Bias>,
    ) -> Result<(Vec<Bias>, usize), ModelNotQuadraticError>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        let obj = model.objective.borrow();
        if obj.has_higher_order() {
            return Err(ModelNotQuadraticError);
        }

        let nvars = obj.num_variables();
        let mut dense: Vec<Bias> = Vec::new();
        dense.resize(nvars * nvars, Bias::default());

        for (u, bias) in obj.linear.iter() {
            dense[u * (nvars + 1)] = *bias;
        }

        if obj.has_quadratic() {
            for (u, v, bias) in obj.quadratic.as_ref().unwrap().iter_flat() {
                dense[u.into() * nvars + v.into()] = bias * 0.5;
                dense[v.into() * nvars + u.into()] = bias * 0.5;
            }
        }

        Ok((dense, nvars))
    }
}
