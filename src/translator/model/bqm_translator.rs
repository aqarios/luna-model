use crate::core::environment::add_variable;
use crate::core::expression::ExpressionBaseAdd;
use crate::core::{ExpressionBaseAdjustment, Sense, Vtype};
use crate::errors::{
    BqmTranslatorErr, ModelSenseNotMinimizeErr, ModelVtypeErr, VariableCreationErr,
};
use crate::{
    core::{
        expression::{BiasConstraints, IndexConstraints},
        ExpressionBase, Model,
    },
    errors::{ModelNotQuadraticErr, ModelNotUnconstrainedErr},
};
use std::rc::Rc;

/// A translator used to read a Binary Quadratic Model (BQM) and create an AQM.
pub struct BqmTranslator {}

impl BqmTranslator {
    /// Translates a BQM to an AQM.
    pub fn model_from_bqm<Index, Bias>(
        vars: Vec<String>,
        vtype: Vtype,
        offset: Bias,
        linear: &[Bias],
        linear_indices: &[u64],
        quadratic: &[Bias],
        quadratic_rows: &[u64],
        quadratic_cols: &[u64],
        name: Option<String>,
    ) -> Result<Model<Index, Bias>, VariableCreationErr>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
    {
        let model = Model::new(name, Some(Sense::Min));
        for var in vars.iter() {
            add_variable(Rc::clone(&model.environment), var, Some(&vtype), None)?;
        }
        model.objective.borrow_mut().resize(vars.len().into());
        model.objective.borrow_mut().add_offset(offset);
        for (&i, &bias) in linear_indices.iter().zip(linear) {
            model
                .objective
                .borrow_mut()
                .add_linear((i as usize).into(), bias);
        }
        for ((&u, &v), &bias) in quadratic_rows
            .iter()
            .zip(quadratic_cols.iter())
            .zip(quadratic)
        {
            model.objective.borrow_mut().add_quadratic(
                (u as usize).into(),
                (v as usize).into(),
                bias,
            );
        }
        Ok(model)
    }

    /// Back(translate) an AQM to a BQM.
    ///
    /// This method is required for interactions with solvers that require the optimization
    /// problem to be expressed as a BQM. We can use the AQM to define our model and send
    /// the information between workers efficiently. The solving process can then use this function
    /// to express the optimization problem in the expected format.
    pub fn model_to_bqm<Index, Bias>(
        model: &Model<Index, Bias>,
    ) -> Result<
        (
            Bias,
            Vec<Bias>,
            Vec<Bias>,
            Vec<i32>,
            Vec<i32>,
            Option<Vtype>,
            Vec<String>,
        ),
        BqmTranslatorErr,
    >
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

        let linear = obj.linear.to_vec().clone();

        let mut quadratic = Vec::new();
        let mut row_indices = Vec::new();
        let mut col_indices = Vec::new();
        if obj.has_quadratic() {
            for (u, v, bias) in obj.quadratic.as_ref().unwrap().iter_flat() {
                row_indices.push(u.into() as i32);
                col_indices.push(v.into() as i32);
                quadratic.push(bias)
            }
        }

        let offset = obj.offset;

        Ok((
            offset,
            linear,
            quadratic,
            row_indices,
            col_indices,
            vtype,
            variables,
        ))
    }
}
