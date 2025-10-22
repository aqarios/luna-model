use crate::core::expression::ExpressionBaseAdd;
use crate::core::{Sense, Vtype};
use crate::errors::{
    BqmTranslatorErr, ModelSenseNotMinimizeErr, ModelVtypeErr, VariableCreationErr,
};
use crate::types::{Bias, VarIndex};
use crate::{
    core::{ExpressionBase, Model},
    errors::{ModelNotQuadraticErr, ModelNotUnconstrainedErr},
};

/// A translator used to read a Binary Quadratic Model (BQM) and create an LunaModel.
pub struct BqmTranslator {}

impl BqmTranslator {
    /// Translates a BQM to an LunaModel.
    pub fn model_from_bqm(
        vars: Vec<String>,
        vtype: Vtype,
        offset: Bias,
        linear: &[Bias],
        linear_indices: &[u64],
        quadratic: &[Bias],
        quadratic_rows: &[u64],
        quadratic_cols: &[u64],
        name: Option<String>,
    ) -> Result<Model, VariableCreationErr> {
        let mut model = Model::new(name, Some(Sense::Min));
        for var in vars.iter() {
            model.environment.add_variable(var, Some(vtype), None)?;
        }
        // model.objective.resize(vars.len().into());
        model.objective.add_offset(offset);
        for (&i, &bias) in linear_indices.iter().zip(linear) {
            model.objective.add_linear((i as usize).into(), bias);
        }
        for ((&u, &v), &bias) in quadratic_rows
            .iter()
            .zip(quadratic_cols.iter())
            .zip(quadratic)
        {
            model
                .objective
                .add_quadratic((u as usize).into(), (v as usize).into(), bias);
        }
        Ok(model)
    }

    /// Back(translate) an LunaModel to a BQM.
    ///
    /// This method is required for interactions with solvers that require the optimization
    /// problem to be expressed as a BQM. We can use the LunaModel to define our model and send
    /// the information between workers efficiently. The solving process can then use this function
    /// to express the optimization problem in the expected format.
    pub fn model_to_bqm(
        model: &Model,
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
    > {
        let obj = &model.objective;
        if obj.has_higher_order() {
            return Err(ModelNotQuadraticErr)?;
        }

        if !model.constraints.is_empty() {
            return Err(ModelNotUnconstrainedErr)?;
        }

        if !model.sense.is_min() {
            return Err(ModelSenseNotMinimizeErr)?;
        }

        let mut vtype = None;
        let mut variables = Vec::with_capacity(model.environment.varcount().into());
        for var in model.environment.access().variables().iter() {
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

        let linear = obj.linear.to_vec(obj.active.len());

        let mut quadratic = Vec::new();
        let mut row_indices = Vec::new();
        let mut col_indices = Vec::new();
        if obj.has_quadratic() {
            for (u, v, bias) in obj.quadratic.as_ref().unwrap().iter_flat() {
                row_indices.push(<VarIndex as Into<usize>>::into(u) as i32);
                col_indices.push(<VarIndex as Into<usize>>::into(v) as i32);
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
