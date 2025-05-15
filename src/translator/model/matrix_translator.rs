use crate::core::Qubo;
use crate::errors::{ModelSenseNotMinimizeErr, ModelVtypeErr, VarNamesErr};
use crate::{
    core::{
        expression::{BiasConstraints, IndexConstraints},
        ExpressionBase, Model, Vtype,
    },
    errors::{MatrixTranslatorErr, ModelNotQuadraticErr, ModelNotUnconstrainedErr},
};

/// A translator used to read a Quadratic Unconstrained Binary Optimization (QUBO) problem
/// and create an AQM.
pub struct MatrixTranslator {}

impl MatrixTranslator {
    /// Translates a QUBO to an AQM.
    pub fn model_from_dense<Index, Bias>(
        name: Option<String>,
        dense: &[Bias],
        num_variables: Index,
        vtype: Option<Vtype>,
        offset: Option<Bias>,
        variable_names: Option<Vec<String>>,
    ) -> Result<Model<Index, Bias>, MatrixTranslatorErr>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        if let Some(names) = variable_names.as_ref() {
            if names.len() != num_variables.into() {
                return Err(VarNamesErr(format!(
                    "Number of variable names must match the number of variables"
                )))?;
            }
        }
        Ok(Model::new_from_dense(
            name,
            vtype,
            dense,
            num_variables,
            offset,
            variable_names,
        )?)
    }

    /// Back(translate) an AQM to a QUBO.
    ///
    /// This method is required for interactions with solvers that require the optimization
    /// problem to be expressed in a QUBO. We can use the AQM to define our model and send
    /// the information between workers efficiently. The solving process can then use this function
    /// to express the optimization problem in the expected format.
    pub fn model_to_dense<Index, Bias>(
        model: &Model<Index, Bias>,
    ) -> Result<Qubo<Index, Bias>, MatrixTranslatorErr>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        let obj = model.objective.borrow();
        if obj.has_higher_order() {
            return Err(ModelNotQuadraticErr)?;
        }

        if !model.constraints.borrow().is_empty() {
            return Err(ModelNotUnconstrainedErr)?;
        }

        if !model.sense.is_min() {
            return Err(ModelSenseNotMinimizeErr)?;
        }

        let mut vtype = None;
        let env = model.environment.borrow();
        let mut variables = Vec::with_capacity(env.varcount.into());
        for var in env.variables.iter() {
            match vtype {
                None => {
                    if var.vtype == Vtype::Integer || var.vtype == Vtype::Real {
                        return Err(ModelVtypeErr(String::from("vtype is not binary or spin")))?;
                    } else {
                        vtype = Some(var.vtype);
                    }
                }
                Some(vt) => {
                    if vt != var.vtype {
                        return Err(ModelVtypeErr(String::from(
                            "variables have different vtypes",
                        )))?;
                    }
                }
            }
            variables.push(var.name.clone());
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

        let qubo = Qubo {
            name: model.name.clone(),
            vtype: vtype.unwrap_or(Vtype::Binary),
            matrix_flat: dense,
            num_variables: nvars.into(),
            offset: obj.offset,
            variable_names: variables,
        };

        Ok(qubo)
    }
}
